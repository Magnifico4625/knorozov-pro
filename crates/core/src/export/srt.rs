//! SRT subtitles.
use crate::timefmt;
use crate::transcript::{Paragraph, Transcript};

/// Max characters per cue (≈ two lines of 42).
const MAX_CUE_CHARS: usize = 84;
const MAX_LINE_CHARS: usize = 42;

#[derive(Debug, Clone, PartialEq)]
pub struct Cue {
    pub start_ms: u64,
    pub end_ms: u64,
    pub text: String,
}

/// Split `text` spanning `[start, end)` into cues of ≤ MAX_CUE_CHARS, distributing time
/// proportionally to character count.
pub fn split_timed(text: &str, start_ms: u64, end_ms: u64) -> Vec<Cue> {
    let words: Vec<&str> = text.split_whitespace().collect();
    if words.is_empty() {
        return vec![];
    }
    let mut chunks: Vec<String> = Vec::new();
    let mut cur = String::new();
    for w in words {
        let would = cur.chars().count() + if cur.is_empty() { 0 } else { 1 } + w.chars().count();
        if !cur.is_empty() && would > MAX_CUE_CHARS {
            chunks.push(std::mem::take(&mut cur));
        }
        if !cur.is_empty() {
            cur.push(' ');
        }
        cur.push_str(w);
    }
    if !cur.is_empty() {
        chunks.push(cur);
    }
    let total_chars: usize = chunks.iter().map(|c| c.chars().count().max(1)).sum();
    let span = end_ms.saturating_sub(start_ms).max(1);
    let mut cues = Vec::with_capacity(chunks.len());
    let mut acc = 0usize;
    for (i, c) in chunks.iter().enumerate() {
        let s = start_ms + (span as u128 * acc as u128 / total_chars as u128) as u64;
        acc += c.chars().count().max(1);
        let e = if i + 1 == chunks.len() {
            end_ms.max(s + 1)
        } else {
            start_ms + (span as u128 * acc as u128 / total_chars as u128) as u64
        };
        cues.push(Cue { start_ms: s, end_ms: e, text: wrap_two_lines(c) });
    }
    cues
}

fn wrap_two_lines(s: &str) -> String {
    if s.chars().count() <= MAX_LINE_CHARS {
        return s.to_string();
    }
    // break at the space closest to the middle
    let chars: Vec<char> = s.chars().collect();
    let mid = chars.len() / 2;
    let best = chars
        .iter()
        .enumerate()
        .filter(|(_, c)| **c == ' ')
        .min_by_key(|(i, _)| (*i as isize - mid as isize).abs())
        .map(|(i, _)| i);
    match best {
        Some(i) => {
            let a: String = chars[..i].iter().collect();
            let b: String = chars[i + 1..].iter().collect();
            format!("{a}\n{b}")
        }
        None => s.to_string(),
    }
}

fn paragraph_cues(p: &Paragraph) -> Vec<Cue> {
    if p.edited || p.segments.is_empty() {
        split_timed(&p.text, p.start_ms, p.end_ms)
    } else {
        p.segments.iter().flat_map(|s| split_timed(&s.text, s.start_ms, s.end_ms)).collect()
    }
}

pub fn cues(t: &Transcript) -> Vec<Cue> {
    let mut out: Vec<Cue> = Vec::new();
    for p in &t.paragraphs {
        let mut pc = paragraph_cues(p);
        if let (Some(first), Some(name)) = (pc.first_mut(), t.speaker_name(p.speaker)) {
            first.text = format!("{name}: {}", first.text);
        }
        out.extend(pc);
    }
    // enforce monotonic, non-overlapping timings
    for i in 1..out.len() {
        if out[i].start_ms < out[i - 1].end_ms {
            out[i - 1].end_ms = out[i].start_ms.max(out[i - 1].start_ms + 1);
        }
        if out[i].end_ms <= out[i].start_ms {
            out[i].end_ms = out[i].start_ms + 1;
        }
    }
    out
}

pub fn to_srt(t: &Transcript) -> String {
    let mut s = String::new();
    for (i, c) in cues(t).iter().enumerate() {
        s.push_str(&format!(
            "{}\n{} --> {}\n{}\n\n",
            i + 1,
            timefmt::srt(c.start_ms),
            timefmt::srt(c.end_ms),
            c.text
        ));
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transcript::Segment;

    #[test]
    fn srt_format_and_timing() {
        let t = crate::export::testdata::sample();
        let srt = to_srt(&t);
        let first: Vec<&str> = srt.lines().take(3).collect();
        assert_eq!(first[0], "1");
        assert_eq!(first[1], "00:12:45,000 --> 00:12:48,500");
        assert!(first[2].starts_with("Иван: Мы сегодня"));
        assert!(srt.contains("Спикер 2: Да, согласен."));
        assert!(srt.contains("00:13:28,000 --> 00:13:32,000"));
    }

    #[test]
    fn cues_monotonic_and_split() {
        let long = "слово ".repeat(60);
        let cs = split_timed(&long, 1_000, 31_000);
        assert!(cs.len() >= 4);
        assert_eq!(cs.first().unwrap().start_ms, 1_000);
        assert_eq!(cs.last().unwrap().end_ms, 31_000);
        for w in cs.windows(2) {
            assert!(w[0].end_ms <= w[1].start_ms);
            assert!(w[0].start_ms < w[0].end_ms);
        }
        for c in &cs {
            for line in c.text.lines() {
                assert!(line.chars().count() <= MAX_LINE_CHARS + 6, "{line}");
            }
            assert!(c.text.lines().count() <= 2);
        }
    }

    #[test]
    fn edited_paragraph_redistributes_time() {
        let mut t = Transcript::from_segments(
            "ru",
            10_000,
            vec![
                Segment { start_ms: 0, end_ms: 2000, text: "раз".into(), speaker: None },
                Segment { start_ms: 2000, end_ms: 4000, text: "два".into(), speaker: None },
            ],
        );
        t.set_paragraph_text(0, "Исправленный текст");
        let cs = cues(&t);
        assert_eq!(cs.len(), 1);
        assert_eq!((cs[0].start_ms, cs[0].end_ms), (0, 4000));
        assert_eq!(cs[0].text, "Исправленный текст");
    }

    #[test]
    fn overlapping_segments_are_fixed() {
        let t = Transcript::from_segments(
            "en",
            10_000,
            vec![
                Segment { start_ms: 0, end_ms: 3000, text: "a".into(), speaker: None },
                Segment { start_ms: 2500, end_ms: 4000, text: "b".into(), speaker: None },
            ],
        );
        let cs = cues(&t);
        assert!(cs[0].end_ms <= cs[1].start_ms);
    }
}
