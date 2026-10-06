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
