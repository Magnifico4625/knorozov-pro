//! Integration test: transcribe short Russian (ogg/opus) and English (mp3) samples with the
//! tiny model, plus diarization of a two-voice mix.
//!
//! Needs `KNOROZOV_TEST_MODEL=/path/to/ggml-tiny.bin`. Without it the tests are skipped, unless
//! `KNOROZOV_REQUIRE_MODEL=1` (set in CI) — then they fail.

use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use knorozov_core::{asr, audio, pipeline};

fn fixtures() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures")
}

fn model() -> Option<PathBuf> {
    match std::env::var("KNOROZOV_TEST_MODEL") {
        Ok(p) if PathBuf::from(&p).exists() => Some(PathBuf::from(p)),
        _ => {
            if std::env::var("KNOROZOV_REQUIRE_MODEL").as_deref() == Ok("1") {
                panic!("KNOROZOV_TEST_MODEL is not set or missing");
            }
            eprintln!("skipping: KNOROZOV_TEST_MODEL not set");
            None
        }
    }
}

fn run(file: &str, lang: &str, diarize: bool) -> (knorozov_core::transcript::Transcript, pipeline::JobStats) {
    let m = model().unwrap();
    let pcm = audio::decode_to_16k_mono(&fixtures().join(file), |_| {}, || false).unwrap();
    let engine = asr::Engine::load(&m).unwrap();
    let spk = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../src-tauri/resources/models")
        .join(knorozov_core::diarize::EMBEDDING_MODEL_FILE);
    let opts = pipeline::JobOptions {
        language: lang.into(),
        diarize,
        embedding_model: if diarize { Some(spk) } else { None },
        threads: None,
    };
    let (t, stats) = pipeline::run(&engine, &pcm, &opts, |_| {}, |_| {}, Arc::new(AtomicBool::new(false))).unwrap();
    eprintln!("{file}: lang={} rtf={:.3}\n{}", t.language, stats.rtf, t.plain_text(true, true));
    (t, stats)
}

#[test]
fn russian_voice_message_opus_autodetect() {
    if model().is_none() {
        return;
    }
    let (t, _) = run("ru_short.ogg", "auto", false);
    assert_eq!(t.language, "ru");
    let text = t.plain_text(false, false).to_lowercase();
    let cyr = text.chars().filter(|c| ('а'..='я').contains(c) || *c == 'ё').count();
    assert!(cyr > 50, "expected Cyrillic text, got: {text}");
    assert!(text.contains("всем") || text.contains("его") || text.contains("привык"), "{text}");
    assert!(t.duration_ms > 11_000 && t.duration_ms < 13_000);
}

#[test]
fn english_mp3() {
    if model().is_none() {
        return;
    }
    let (t, _) = run("en_short.mp3", "en", false);
    assert_eq!(t.language, "en");
    let text = t.plain_text(false, false).to_lowercase();
    assert!(text.contains("country"), "{text}");
    assert!(text.contains("ask"), "{text}");
}

#[cfg(feature = "diarization")]
#[test]
fn diarization_two_voices() {
    if model().is_none() {
        return;
    }
    let (t, _) = run("dialog.mp3", "auto", true);
    assert!(t.speakers.len() >= 2, "expected ≥2 speakers, got {:?}", t.speakers);
    let first = t.paragraphs.first().unwrap().speaker;
    let last = t.paragraphs.last().unwrap().speaker;
    assert_eq!(first, last, "the Russian reader returns at the end");
}

#[test]
fn decodes_all_fixtures() {
    for f in ["ru_short.ogg", "en_short.mp3", "dialog.mp3"] {
        let pcm = audio::decode_to_16k_mono(&fixtures().join(f), |_| {}, || false).unwrap();
        assert!(pcm.len() > 16_000 * 5, "{f}");
        let peak = pcm.iter().fold(0f32, |m, v| m.max(v.abs()));
        assert!(peak > 0.05 && peak <= 1.5, "{f} peak {peak}");
    }
}

/// Windows users often have Cyrillic profile names (C:\Users\Дамир) — models must still load.
#[test]
fn models_load_from_cyrillic_paths() {
    let Some(m) = model() else { return };
    let dir = std::env::temp_dir().join(format!("Кнорозов тест {}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let wm = dir.join("модель.bin");
    std::fs::copy(&m, &wm).unwrap();
    let engine = asr::Engine::load(&wm).expect("whisper model from Cyrillic path");
    let pcm = audio::decode_to_16k_mono(&fixtures().join("en_short.mp3"), |_| {}, || false).unwrap();
    let res = engine
        .transcribe(
            &pcm,
            &asr::AsrOptions { language: "en".into(), threads: None },
            |_| {},
            |_| {},
            Arc::new(AtomicBool::new(false)),
        )
        .unwrap();
    assert!(!res.segments.is_empty());

    #[cfg(feature = "diarization")]
    {
        let spk_src = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../src-tauri/resources/models")
            .join(knorozov_core::diarize::EMBEDDING_MODEL_FILE);
        let spk = dir.join("спикеры.onnx");
        std::fs::copy(&spk_src, &spk).unwrap();
        let mut segs = res.segments.clone();
        let n = knorozov_core::diarize::assign_speakers(&spk, &pcm, &mut segs, 2, |_| {}, || false)
            .expect("speaker model from Cyrillic path");
        assert!(n >= 1);
    }
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn cancel_stops_transcription() {
    let Some(m) = model() else { return };
    let engine = asr::Engine::load(&m).unwrap();
    let pcm = audio::decode_to_16k_mono(&fixtures().join("dialog.mp3"), |_| {}, || false).unwrap();
    let cancel = Arc::new(AtomicBool::new(true));
    let r = engine.transcribe(&pcm, &asr::AsrOptions { language: "auto".into(), threads: None }, |_| {}, |_| {}, cancel);
    assert!(r.is_err(), "cancelled job must return an error");
}
