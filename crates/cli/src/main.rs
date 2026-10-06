//! knorozov-cli — headless harness for benchmarks and debugging.
//!
//! Usage:
//!   knorozov-cli transcribe <model.bin> <audio> [--lang ru|auto] [--threads N] [--diarize <spk.onnx>] [--json]
//!   knorozov-cli download <fast|accurate|tiny> <dir> [--mirror URL]
//!   knorozov-cli export <audio> <model.bin> <out.{docx,pdf,txt,srt}>

use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::time::Instant;

use anyhow::{bail, Context, Result};
use knorozov_core::{asr, audio, export, models, pipeline};

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter().position(|a| a == name).and_then(|i| args.get(i + 1).cloned())
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("transcribe") | Some("export") => {
            let is_export = args[1] == "export";
            let (model, input) = if is_export {
                (PathBuf::from(&args[3]), PathBuf::from(&args[2]))
            } else {
                (PathBuf::from(args.get(2).context("model")?), PathBuf::from(args.get(3).context("audio")?))
            };
            let lang = arg_value(&args, "--lang").unwrap_or_else(|| "auto".into());
            let threads = arg_value(&args, "--threads").and_then(|t| t.parse().ok());
            let diar = arg_value(&args, "--diarize").map(PathBuf::from);

            let t = Instant::now();
            let pcm = audio::decode_to_16k_mono(&input, |_| {}, || false)?;
            let decode_sec = t.elapsed().as_secs_f64();
            let t = Instant::now();
            let engine = asr::Engine::load(&model)?;
            let load_sec = t.elapsed().as_secs_f64();
            let opts = pipeline::JobOptions {
                language: lang,
                diarize: diar.is_some(),
                embedding_model: diar,
                threads,
            };
            let (tr, stats) = pipeline::run(&engine, &pcm, &opts, |_| {}, |_| {}, Arc::new(AtomicBool::new(false)))?;
            if is_export {
                let out = PathBuf::from(&args[4]);
                let fmt = match out.extension().and_then(|e| e.to_str()) {
                    Some("docx") => export::ExportFormat::Docx,
                    Some("pdf") => export::ExportFormat::Pdf,
                    Some("txt") => export::ExportFormat::Txt,
                    Some("srt") => export::ExportFormat::Srt,
                    _ => bail!("unknown extension"),
                };
                let meta = export::DocMeta {
                    title: input.file_name().unwrap().to_string_lossy().into(),
                    subtitle: knorozov_core::timefmt::duration(tr.duration_ms),
                };
                std::fs::write(&out, export::export_bytes(&tr, &meta, fmt)?)?;
                println!("wrote {}", out.display());
                return Ok(());
            }
            if args.iter().any(|a| a == "--json") {
                println!(
                    "{}",
                    serde_json::json!({
                        "file": input.file_name().unwrap().to_string_lossy(),
                        "model": model.file_name().unwrap().to_string_lossy(),
                        "language": tr.language,
                        "audio_sec": stats.audio_sec,
                        "decode_sec": decode_sec,
                        "load_sec": load_sec,
                        "asr_sec": stats.asr_sec,
                        "diarize_sec": stats.diarize_sec,
                        "rtf": stats.rtf,
                        "speakers": tr.speakers.len(),
                        "threads": threads.unwrap_or_else(asr::default_threads),
                        "text": tr.plain_text(true, false),
                    })
                );
            } else {
                println!("{}", tr.plain_text(true, true));
                eprintln!(
                    "lang={} audio={:.1}s decode={:.2}s load={:.2}s asr={:.2}s diarize={:.2}s RTF={:.3} speakers={}",
                    tr.language, stats.audio_sec, decode_sec, load_sec, stats.asr_sec, stats.diarize_sec, stats.rtf,
                    tr.speakers.len()
                );
            }
        }
        Some("download") => {
            let q = match args.get(2).map(|s| s.as_str()) {
                Some("fast") => models::Quality::Fast,
                Some("accurate") => models::Quality::Accurate,
                Some("tiny") => models::Quality::Tiny,
                _ => bail!("quality: fast|accurate|tiny"),
            };
            let dir = PathBuf::from(args.get(3).context("dir")?);
            let mirror = arg_value(&args, "--mirror").unwrap_or_default();
            let cancel = AtomicBool::new(false);
            let p = models::download(&dir, q, &mirror, &cancel, |p| {
                eprint!("\r{:.1}% {:.1} MB/s   ", p.downloaded as f64 * 100.0 / p.total as f64, p.speed / 1e6);
            })?;
            eprintln!();
            println!("{}", p.display());
        }
        _ => {
            eprintln!("usage: knorozov-cli transcribe <model> <audio> [--lang xx] [--threads N] [--diarize spk.onnx] [--json]");
            eprintln!("       knorozov-cli download <fast|accurate|tiny> <dir> [--mirror URL]");
            eprintln!("       knorozov-cli export <audio> <model> <out.docx|pdf|txt|srt>");
        }
    }
    Ok(())
}
