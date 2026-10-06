//! Speaker diarization («Разделить по спикерам»), best-effort.
//!
//! License-clean approach without a segmentation model: every Whisper segment is a speech window;
//! we compute a CAM++ speaker embedding (3D-Speaker, Apache-2.0) with sherpa-onnx for each window
//! and cluster them (agglomerative, average linkage, cosine similarity).

use crate::transcript::Segment;

pub const EMBEDDING_MODEL_FILE: &str = "3dspeaker_speech_campplus_sv_zh_en_16k-common_advanced.onnx";
/// Average-linkage cosine similarity above which two clusters are merged.
pub const DEFAULT_THRESHOLD: f32 = 0.55;
const MIN_WINDOW_MS: u64 = 600;
const MAX_WINDOW_MS: u64 = 15_000;

fn cosine(a: &[f32], b: &[f32]) -> f32 {
    let (mut dot, mut na, mut nb) = (0f32, 0f32, 0f32);
    for (x, y) in a.iter().zip(b) {
        dot += x * y;
        na += x * x;
        nb += y * y;
    }
    if na == 0.0 || nb == 0.0 {
        0.0
    } else {
        dot / (na.sqrt() * nb.sqrt())
    }
}

/// Agglomerative clustering with average linkage. Returns a cluster label per input, labels are
/// renumbered in order of first appearance (so the first voice is «Спикер 1»).
pub fn cluster(embeddings: &[Vec<f32>], threshold: f32, max_speakers: usize) -> Vec<usize> {
    let n = embeddings.len();
    if n == 0 {
        return vec![];
    }
    let mut sim = vec![vec![0f32; n]; n];
    for i in 0..n {
        for j in i + 1..n {
            let s = cosine(&embeddings[i], &embeddings[j]);
            sim[i][j] = s;
            sim[j][i] = s;
        }
    }
    let mut clusters: Vec<Vec<usize>> = (0..n).map(|i| vec![i]).collect();
    // cluster-level average similarity matrix
    let mut csim = sim.clone();
    loop {
        let k = clusters.len();
        if k <= 1 {
            break;
        }
        let mut best = (f32::MIN, 0, 0);
        for a in 0..k {
            for b in a + 1..k {
                if csim[a][b] > best.0 {
                    best = (csim[a][b], a, b);
                }
            }
        }
        if best.0 < threshold && k <= max_speakers {
            break;
        }
        let (_, a, b) = best;
        let (na, nb) = (clusters[a].len() as f32, clusters[b].len() as f32);
        // merge b into a, update average linkage
        for c in 0..k {
            if c != a && c != b {
                let v = (csim[a][c] * na + csim[b][c] * nb) / (na + nb);
                csim[a][c] = v;
                csim[c][a] = v;
            }
        }
        let moved = clusters.remove(b);
        clusters[a].extend(moved);
        csim.remove(b);
        for row in csim.iter_mut() {
            row.remove(b);
        }
    }
    let mut label = vec![0usize; n];
    for (ci, members) in clusters.iter().enumerate() {
        for &m in members {
            label[m] = ci;
        }
    }
    // renumber by first appearance
    let mut map: Vec<Option<usize>> = vec![None; clusters.len()];
    let mut next = 0;
    for l in label.iter_mut() {
        let new = *map[*l].get_or_insert_with(|| {
            next += 1;
            next - 1
        });
        *l = new;
    }
    label
}

/// Clusters with very little total speech (< `MIN_SPEAKER_MS`) are usually noise / short
/// interjections — reassign their members to the most similar remaining cluster centroid.
pub fn merge_small_clusters(embs: &[Vec<f32>], durations_ms: &[u64], labels: Vec<usize>) -> Vec<usize> {
    const MIN_SPEAKER_MS: u64 = 4_000;
    let k = labels.iter().max().map(|m| m + 1).unwrap_or(0);
    if k <= 1 {
        return labels;
    }
    let mut total = vec![0u64; k];
    for (l, d) in labels.iter().zip(durations_ms) {
        total[*l] += d;
    }
    let big: Vec<usize> = (0..k).filter(|&c| total[c] >= MIN_SPEAKER_MS).collect();
    if big.is_empty() || big.len() == k {
        return labels;
    }
    let dim = embs[0].len();
    let centroid = |c: usize| {
        let mut v = vec![0f32; dim];
        for (e, l) in embs.iter().zip(&labels) {
            if *l == c {
                for (a, b) in v.iter_mut().zip(e) {
                    *a += b;
                }
            }
        }
        v
    };
    let cents: Vec<(usize, Vec<f32>)> = big.iter().map(|&c| (c, centroid(c))).collect();
    let mut out: Vec<usize> = labels
        .iter()
        .zip(embs)
        .map(|(&l, e)| {
            if big.contains(&l) {
                l
            } else {
                cents
                    .iter()
                    .max_by(|a, b| cosine(e, &a.1).total_cmp(&cosine(e, &b.1)))
                    .map(|c| c.0)
                    .unwrap_or(l)
            }
        })
        .collect();
    // renumber by first appearance
    let mut map = std::collections::HashMap::new();
    for l in out.iter_mut() {
        let n = map.len();
        *l = *map.entry(*l).or_insert(n);
    }
    out
}

/// Give speaker-less (too short) segments the label of their nearest labelled neighbour.
pub fn fill_gaps(segments: &mut [Segment]) {
    let labelled: Vec<(usize, usize)> =
        segments.iter().enumerate().filter_map(|(i, s)| s.speaker.map(|sp| (i, sp))).collect();
    if labelled.is_empty() {
        return;
    }
    for i in 0..segments.len() {
        if segments[i].speaker.is_none() {
            let nearest = labelled.iter().min_by_key(|(j, _)| (*j as isize - i as isize).unsigned_abs()).unwrap();
            segments[i].speaker = Some(nearest.1);
        }
    }
}

#[cfg(feature = "diarization")]
pub use imp::*;

#[cfg(feature = "diarization")]
mod imp {
    use super::*;
    use anyhow::{anyhow, Result};
    use std::path::Path;

    /// Assign `speaker` to each segment in place. `audio` is 16 kHz mono.
    pub fn assign_speakers(
        model: &Path,
        audio: &[f32],
        segments: &mut [Segment],
        threads: usize,
        mut on_progress: impl FnMut(f32),
        should_cancel: impl Fn() -> bool,
    ) -> Result<usize> {
        let mut extractor = sherpa_rs::speaker_id::EmbeddingExtractor::new(sherpa_rs::speaker_id::ExtractorConfig {
            model: model.to_string_lossy().into_owned(),
            provider: Some("cpu".into()),
            num_threads: Some(threads.max(1)),
            debug: false,
        })
        .map_err(|e| anyhow!("модель спикеров: {e}"))?;
        let mut idx = Vec::new();
        let mut embs = Vec::new();
        let total = segments.len().max(1);
        for (i, s) in segments.iter().enumerate() {
            if should_cancel() {
                anyhow::bail!("отменено");
            }
            let dur = s.end_ms.saturating_sub(s.start_ms);
            if dur < MIN_WINDOW_MS {
                continue;
            }
            let a = (s.start_ms * 16) as usize;
            let b = ((s.start_ms + dur.min(MAX_WINDOW_MS)) * 16) as usize;
            let (a, b) = (a.min(audio.len()), b.min(audio.len()));
            if b <= a + 16 * MIN_WINDOW_MS as usize / 2 {
                continue;
            }
            match extractor.compute_speaker_embedding(audio[a..b].to_vec(), 16_000) {
                Ok(e) => {
                    idx.push(i);
                    embs.push(e);
                }
                Err(e) => log::warn!("embedding failed for segment {i}: {e}"),
            }
            on_progress((i + 1) as f32 / total as f32);
        }
        let durations: Vec<u64> = idx.iter().map(|&i| segments[i].end_ms.saturating_sub(segments[i].start_ms)).collect();
        let labels = merge_small_clusters(&embs, &durations, cluster(&embs, DEFAULT_THRESHOLD, 8));
        for s in segments.iter_mut() {
            s.speaker = None;
        }
        for (k, &i) in idx.iter().enumerate() {
            segments[i].speaker = Some(labels[k]);
        }
        fill_gaps(segments);
        Ok(labels.iter().max().map(|m| m + 1).unwrap_or(0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clusters_two_voices() {
        let a = vec![1.0, 0.1, 0.0];
        let a2 = vec![0.9, 0.2, 0.05];
        let b = vec![0.0, 0.1, 1.0];
        let b2 = vec![0.05, 0.0, 0.95];
        let labels = cluster(&[b.clone(), a, b2, a2, b], 0.55, 8);
        assert_eq!(labels, vec![0, 1, 0, 1, 0]);
    }

    #[test]
    fn single_voice_one_cluster() {
        let v: Vec<Vec<f32>> = (0..6).map(|i| vec![1.0, 0.05 * i as f32, 0.0]).collect();
        assert!(cluster(&v, 0.55, 8).iter().all(|&l| l == 0));
    }

    #[test]
    fn caps_max_speakers() {
        let v: Vec<Vec<f32>> = (0..5)
            .map(|i| {
                let mut e = vec![0.0; 5];
                e[i] = 1.0;
                e
            })
            .collect();
        let labels = cluster(&v, 0.9, 2);
        assert!(labels.iter().max().unwrap() <= &1);
    }

    #[test]
    fn small_clusters_are_merged() {
        let embs = vec![vec![1.0, 0.0], vec![0.0, 1.0], vec![0.9, 0.3], vec![0.1, 0.9]];
        let labels = vec![0, 1, 2, 1];
        let d = vec![5000, 5000, 1500, 5000];
        assert_eq!(merge_small_clusters(&embs, &d, labels), vec![0, 1, 0, 1]);
    }

    #[test]
    fn gap_fill() {
        let mut s = vec![
            Segment { start_ms: 0, end_ms: 1000, text: "a".into(), speaker: Some(1) },
            Segment { start_ms: 1000, end_ms: 1100, text: "b".into(), speaker: None },
            Segment { start_ms: 1100, end_ms: 3000, text: "c".into(), speaker: Some(0) },
        ];
        fill_gaps(&mut s);
        assert!(s[1].speaker.is_some());
    }
}
