//! End-to-end job: (decoded audio) → whisper → optional diarization → Transcript.
//! Shared by the desktop app and the CLI benchmark harness.

use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Instant;

use anyhow::Result;
use serde::Serialize;

use crate::asr::{AsrOptions, Engine, LiveSegment};
use crate::transcript::Transcript;

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Stage {
    Decoding,
    Loading,
    Transcribing,
    Diarizing,
    Saving,
}

#[derive(Debug, Clone, Serialize)]
pub struct Progress {
    pub stage: Stage,
    /// Overall 0..=100.
    pub percent: f32,
    /// Estimated seconds remaining (None until we have a rate).
    pub eta_sec: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct JobOptions {
    pub language: String,
    pub diarize: bool,
    pub embedding_model: Option<PathBuf>,
    pub threads: Option<usize>,
}

/// Overall progress split: decoding 0–5, transcription 5–95 (or 5–90 with diarization), rest diarization.
pub fn transcription_span(diarize: bool) -> (f32, f32) {
    if diarize {
        (5.0, 90.0)
    } else {
        (5.0, 99.0)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct JobStats {
    pub audio_sec: f64,
    pub asr_sec: f64,
    pub diarize_sec: f64,
    pub rtf: f64,
}

pub fn run(
    engine: &Engine,
    audio: &[f32],
    opts: &JobOptions,
    on_progress: impl FnMut(Progress) + Send + 'static,
    on_segment: impl FnMut(LiveSegment) + Send + 'static,
    cancel: Arc<AtomicBool>,
) -> Result<(Transcript, JobStats)> {
    let audio_sec = audio.len() as f64 / 16_000.0;
    let (lo, hi) = transcription_span(opts.diarize);
    let on_progress = Arc::new(Mutex::new(on_progress));
    let t0 = Instant::now();
    {
        let cb = on_progress.clone();
        let started = Instant::now();
        let asr_cb = move |p: i32| {
            let p = p.clamp(0, 100) as f32;
            let elapsed = started.elapsed().as_secs_f64();
            let eta = if p >= 3.0 { Some(elapsed / p as f64 * (100.0 - p as f64)) } else { None };
            if let Ok(mut f) = cb.lock() {
                f(Progress { stage: Stage::Transcribing, percent: lo + (hi - lo) * p / 100.0, eta_sec: eta });
            }
        };
        let asr = engine.transcribe(
            audio,
            &AsrOptions { language: opts.language.clone(), threads: opts.threads },
            asr_cb,
            on_segment,
            cancel.clone(),
        )?;
        let asr_sec = t0.elapsed().as_secs_f64();
        let mut segments = asr.segments;
        let mut diarize_sec = 0.0;

        #[cfg(feature = "diarization")]
        if opts.diarize {
            if let Some(model) = &opts.embedding_model {
                let t1 = Instant::now();
                let cb = on_progress.clone();
                let cancel2 = cancel.clone();
                let res = crate::diarize::assign_speakers(
                    model,
                    audio,
                    &mut segments,
                    opts.threads.unwrap_or_else(crate::asr::default_threads),
                    move |f| {
                        if let Ok(mut g) = cb.lock() {
                            g(Progress { stage: Stage::Diarizing, percent: hi + (99.0 - hi) * f, eta_sec: None });
                        }
                    },
                    move || cancel2.load(Ordering::Relaxed),
                );
                if let Err(e) = res {
                    if cancel.load(Ordering::Relaxed) {
                        return Err(e);
                    }
                    // diarization is best-effort: keep the transcript without speakers
                    log::warn!("diarization failed: {e:#}");
                    for s in segments.iter_mut() {
                        s.speaker = None;
                    }
                }
                diarize_sec = t1.elapsed().as_secs_f64();
            }
        }
        #[cfg(not(feature = "diarization"))]
        let _ = &cancel;

        let duration_ms = (audio_sec * 1000.0) as u64;
        let t = Transcript::from_segments(&asr.language, duration_ms, segments);
        let total = t0.elapsed().as_secs_f64();
        Ok((t, JobStats { audio_sec, asr_sec, diarize_sec, rtf: if audio_sec > 0.0 { total / audio_sec } else { 0.0 } }))
    }
}
