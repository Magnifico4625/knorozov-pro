//! Speech recognition with whisper.cpp (via `whisper-rs`).

use std::ffi::c_void;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use anyhow::{anyhow, bail, Result};
use serde::Serialize;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

use crate::transcript::Segment;

#[derive(Debug, Clone)]
pub struct AsrOptions {
    /// Whisper language code or "auto".
    pub language: String,
    pub threads: Option<usize>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AsrResult {
    pub language: String,
    pub segments: Vec<Segment>,
}

/// Live segment delivered while transcription is running.
#[derive(Debug, Clone, Serialize)]
pub struct LiveSegment {
    pub start_ms: u64,
    pub end_ms: u64,
    pub text: String,
}

pub fn default_threads() -> usize {
    std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4).clamp(1, 8)
}

static HOOKS: std::sync::Once = std::sync::Once::new();

/// A loaded model. Loading takes a few seconds, so the app caches it.
pub struct Engine {
    ctx: WhisperContext,
    pub model_path: PathBuf,
}

unsafe extern "C" fn abort_trampoline(data: *mut c_void) -> bool {
    if data.is_null() {
        return false;
    }
    let flag = unsafe { &*(data as *const AtomicBool) };
    flag.load(Ordering::Relaxed)
}

impl Engine {
    pub fn load(model_path: &Path) -> Result<Self> {
        HOOKS.call_once(whisper_rs::install_logging_hooks);
        if !model_path.exists() {
            bail!("модель не найдена: {}", model_path.display());
        }
        let mut params = WhisperContextParameters::default();
        // Metal is used automatically when the crate is built with the `metal` feature.
        params.flash_attn = cfg!(target_os = "macos");
        // Load from a memory map instead of a path: whisper.cpp opens files with the ANSI code
        // page on Windows, which breaks on Cyrillic user names (C:\Users\Дамир\...).
        let file = std::fs::File::open(model_path)?;
        let map = unsafe { memmap2::Mmap::map(&file)? };
        let ctx = WhisperContext::new_from_buffer_with_params(&map, params)
            .map_err(|e| anyhow!("не удалось загрузить модель: {e:?}"))?;
        Ok(Self { ctx, model_path: model_path.to_path_buf() })
    }

    /// Transcribe 16 kHz mono PCM.
    ///
    /// * `on_progress` — 0..=100 from whisper.cpp
    /// * `on_segment` — each new segment as soon as it is decoded (live text)
    /// * `cancel` — set to true to abort as soon as possible
    pub fn transcribe(
        &self,
        audio: &[f32],
        opts: &AsrOptions,
        on_progress: impl FnMut(i32) + Send + 'static,
        on_segment: impl FnMut(LiveSegment) + Send + 'static,
        cancel: Arc<AtomicBool>,
    ) -> Result<AsrResult> {
        let mut state = self.ctx.create_state().map_err(|e| anyhow!("whisper state: {e:?}"))?;
        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
        let lang = opts.language.trim().to_string();
        let lang_ref: Option<&str> = if lang.is_empty() || lang == "auto" { Some("auto") } else { Some(lang.as_str()) };
        params.set_language(lang_ref);
        params.set_translate(false);
        params.set_n_threads(opts.threads.unwrap_or_else(default_threads) as i32);
        params.set_print_special(false);
        params.set_print_progress(false);
        params.set_print_realtime(false);
        params.set_print_timestamps(false);
        params.set_suppress_blank(true);
        params.set_suppress_nst(true);

        let on_progress = Mutex::new(on_progress);
        params.set_progress_callback_safe(move |p: i32| {
            if let Ok(mut f) = on_progress.lock() {
                f(p)
            }
        });
        let on_segment = Mutex::new(on_segment);
        params.set_segment_callback_safe_lossy(move |d: whisper_rs::SegmentCallbackData| {
            if let Ok(mut f) = on_segment.lock() {
                f(LiveSegment {
                    start_ms: (d.start_timestamp.max(0) as u64) * 10,
                    end_ms: (d.end_timestamp.max(0) as u64) * 10,
                    text: d.text.trim().to_string(),
                })
            }
        });
        // NB: we use our own trampoline instead of `set_abort_callback_safe`.
        let flag_ptr = Arc::as_ptr(&cancel) as *mut c_void;
        unsafe {
            params.set_abort_callback(Some(abort_trampoline));
            params.set_abort_callback_user_data(flag_ptr);
        }

        let res = state.full(params, audio);
        if cancel.load(Ordering::Relaxed) {
            bail!("отменено");
        }
        res.map_err(|e| anyhow!("ошибка распознавания: {e:?}"))?;

        let lang_id = state.full_lang_id_from_state();
        let language = whisper_rs::get_lang_str(lang_id).unwrap_or("auto").to_string();
        let mut segments = Vec::new();
        for seg in state.as_iter() {
            let text = seg.to_str_lossy().map(|c| c.trim().to_string()).unwrap_or_default();
            segments.push(Segment {
                start_ms: seg.start_timestamp().max(0) as u64 * 10,
                end_ms: seg.end_timestamp().max(0) as u64 * 10,
                text,
                speaker: None,
            });
        }
        Ok(AsrResult { language, segments })
    }
}
