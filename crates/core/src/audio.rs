//! Decoding of audio/video files into 16 kHz mono `f32` PCM (what Whisper expects).
//!
//! Pure Rust (symphonia) for mp3 / aac(m4a, mp4, mov) / wav / ogg-vorbis / flac,
//! plus libopus (BSD-3) via `symphonia-adapter-libopus` for ogg/opus voice messages.
//! If decoding fails and a system `ffmpeg` is available on PATH, it is used as a fallback
//! (ffmpeg is never bundled).

use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::OnceLock;

use anyhow::{anyhow, bail, Context, Result};
use rubato::{FftFixedIn, Resampler};
use serde::Serialize;
use symphonia::core::codecs::audio::AudioDecoderOptions;
use symphonia::core::codecs::registry::CodecRegistry;
use symphonia::core::errors::Error as SymError;
use symphonia::core::formats::probe::Hint;
use symphonia::core::formats::{FormatOptions, FormatReader, TrackType};
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;

pub const WHISPER_SAMPLE_RATE: u32 = 16_000;

/// File extensions accepted in the UI.
pub const SUPPORTED_EXTENSIONS: &[&str] = &[
    "mp3", "m4a", "wav", "ogg", "oga", "opus", "mp4", "mov", "m4v", "flac", "aac", "mkv", "webm",
    "caf",
];

fn codecs() -> &'static CodecRegistry {
    static REG: OnceLock<CodecRegistry> = OnceLock::new();
    REG.get_or_init(|| {
        let mut reg = CodecRegistry::new();
        symphonia::default::register_enabled_codecs(&mut reg);
        reg.register_audio_decoder::<symphonia_adapter_libopus::OpusDecoder>();
        reg
    })
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct MediaInfo {
    /// Lower-case extension (e.g. "mp4").
    pub format: String,
    /// Duration in seconds if the container tells us.
    pub duration_sec: Option<f64>,
    pub sample_rate: Option<u32>,
    pub channels: Option<usize>,
}

fn ext_of(path: &Path) -> String {
    path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase()
}

fn open_format(path: &Path) -> Result<Box<dyn FormatReader>> {
    let file = File::open(path).with_context(|| format!("не удалось открыть файл {}", path.display()))?;
    let mss = MediaSourceStream::new(Box::new(file), Default::default());
    let mut hint = Hint::new();
    let ext = ext_of(path);
    if !ext.is_empty() {
        hint.with_extension(&ext);
    }
    let format = symphonia::default::get_probe()
        .probe(&hint, mss, FormatOptions::default(), MetadataOptions::default())
        .map_err(|e| anyhow!("неподдерживаемый формат: {e}"))?;
    Ok(format)
}

/// Quickly read container metadata (no full decode).
pub fn probe(path: &Path) -> Result<MediaInfo> {
    let format = open_format(path)?;
    let track = format
        .default_track(TrackType::Audio)
        .ok_or_else(|| anyhow!("в файле нет звуковой дорожки"))?;
    let audio = track.codec_params.as_ref().and_then(|p| p.audio());
    let sample_rate = audio.and_then(|a| a.sample_rate);
    let channels = audio.and_then(|a| a.channels.as_ref().map(|c| c.count()));
    let duration_sec = match (track.num_frames, sample_rate) {
        (Some(n), Some(sr)) if sr > 0 && n > 0 => Some(n as f64 / sr as f64),
        _ => None,
    };
    Ok(MediaInfo { format: ext_of(path), duration_sec, sample_rate, channels })
}

/// Decode any supported file to 16 kHz mono f32.
///
/// `on_progress` receives a 0.0..=1.0 fraction (best effort, based on container duration).
/// `should_cancel` is polled regularly.
pub fn decode_to_16k_mono(
    path: &Path,
    mut on_progress: impl FnMut(f32),
    should_cancel: impl Fn() -> bool,
) -> Result<Vec<f32>> {
    match decode_symphonia(path, &mut on_progress, &should_cancel) {
        Ok(v) if !v.is_empty() => Ok(v),
        Ok(_) => decode_with_system_ffmpeg(path).context("файл не содержит звука"),
        Err(e) => {
            if should_cancel() {
                return Err(e);
            }
            log::warn!("symphonia failed ({e:#}); trying system ffmpeg");
            decode_with_system_ffmpeg(path).map_err(|fe| {
                anyhow!("не удалось декодировать файл: {e:#} (ffmpeg: {fe:#})")
            })
        }
    }
}

fn decode_symphonia(
    path: &Path,
    on_progress: &mut impl FnMut(f32),
    should_cancel: &impl Fn() -> bool,
) -> Result<Vec<f32>> {
    let mut format = open_format(path)?;
    let track = format
        .default_track(TrackType::Audio)
        .ok_or_else(|| anyhow!("в файле нет звуковой дорожки"))?;
    let track_id = track.id;
    let params = track
        .codec_params
        .as_ref()
        .and_then(|p| p.audio())
        .ok_or_else(|| anyhow!("нет параметров аудиокодека"))?
        .clone();
    let total_frames = track.num_frames;
    let mut decoder = codecs()
        .make_audio_decoder(&params, &AudioDecoderOptions::default())
        .map_err(|e| anyhow!("неподдерживаемый аудиокодек: {e}"))?;

    let mut mono: Vec<f32> = Vec::new();
    let mut interleaved: Vec<f32> = Vec::new();
    let mut src_rate: Option<u32> = params.sample_rate;
    let mut frames_done: u64 = 0;
    let mut last_report = 0u64;

    loop {
        if should_cancel() {
            bail!("отменено");
        }
        let packet = match format.next_packet() {
            Ok(Some(p)) => p,
            Ok(None) => break,
            Err(SymError::ResetRequired) => break,
            Err(SymError::IoError(e)) if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
            Err(e) => {
                if mono.is_empty() {
                    return Err(anyhow!("ошибка чтения: {e}"));
                }
                break;
            }
        };
        if packet.track_id != track_id {
            continue;
        }
        let buf = match decoder.decode(&packet) {
            Ok(b) => b,
            Err(SymError::DecodeError(_)) | Err(SymError::IoError(_)) => continue,
            Err(e) => return Err(anyhow!("ошибка декодирования: {e}")),
        };
        let spec = buf.spec().clone();
        let ch = spec.channels().count().max(1);
        src_rate.get_or_insert(spec.rate());
        if src_rate != Some(spec.rate()) {
            // Sample-rate change mid-stream is extremely rare; keep the first rate.
            log::warn!("sample rate changed mid-stream");
        }
        interleaved.resize(buf.samples_interleaved(), 0.0);
        buf.copy_to_slice_interleaved(&mut interleaved[..]);
        let frames = interleaved.len() / ch;
        mono.reserve(frames);
        if ch == 1 {
            mono.extend_from_slice(&interleaved);
        } else {
            let inv = 1.0 / ch as f32;
            for f in interleaved.chunks_exact(ch) {
                mono.push(f.iter().sum::<f32>() * inv);
            }
        }
        frames_done += frames as u64;
        if let Some(total) = total_frames {
            if frames_done - last_report > 48_000 * 5 {
                last_report = frames_done;
                on_progress((frames_done as f32 / total as f32).min(1.0) * 0.8);
            }
        }
    }
    let rate = src_rate.ok_or_else(|| anyhow!("неизвестная частота дискретизации"))?;
    let out = resample_to_16k(&mono, rate)?;
    on_progress(1.0);
    Ok(out)
}

/// High-quality FFT resampling of a mono signal to 16 kHz.
pub fn resample_to_16k(input: &[f32], src_rate: u32) -> Result<Vec<f32>> {
    if src_rate == WHISPER_SAMPLE_RATE || input.is_empty() {
        return Ok(input.to_vec());
    }
    const CHUNK: usize = 4096;
    let mut rs = FftFixedIn::<f32>::new(src_rate as usize, WHISPER_SAMPLE_RATE as usize, CHUNK, 2, 1)
        .map_err(|e| anyhow!("resampler: {e}"))?;
    let delay = rs.output_delay();
    let expected = (input.len() as u64 * WHISPER_SAMPLE_RATE as u64 / src_rate as u64) as usize;
    let mut out = Vec::with_capacity(expected + CHUNK);
    let mut pos = 0;
    while input.len() - pos >= rs.input_frames_next() {
        let n = rs.input_frames_next();
        let res = rs
            .process(&[&input[pos..pos + n]], None)
            .map_err(|e| anyhow!("resample: {e}"))?;
        out.extend_from_slice(&res[0]);
        pos += n;
    }
    if pos < input.len() {
        let res = rs
            .process_partial(Some(&[&input[pos..]]), None)
            .map_err(|e| anyhow!("resample: {e}"))?;
        out.extend_from_slice(&res[0]);
    }
    // flush the filter delay
    let res = rs
        .process_partial::<&[f32]>(None, None)
        .map_err(|e| anyhow!("resample: {e}"))?;
    out.extend_from_slice(&res[0]);
    let start = delay.min(out.len());
    let mut out = out.split_off(start);
    out.truncate(expected);
    Ok(out)
}

/// Fallback: use an ffmpeg binary found on PATH (not bundled with the app).
fn decode_with_system_ffmpeg(path: &Path) -> Result<Vec<f32>> {
    let output = std::process::Command::new("ffmpeg")
        .args(["-nostdin", "-hide_banner", "-loglevel", "error", "-i"])
        .arg(path)
        .args(["-vn", "-ac", "1", "-ar", "16000", "-f", "f32le", "-"])
        .output()
        .context("ffmpeg не найден")?;
    if !output.status.success() {
        bail!("ffmpeg: {}", String::from_utf8_lossy(&output.stderr).trim());
    }
    Ok(output
        .stdout
        .chunks_exact(4)
        .map(|b| f32::from_le_bytes([b[0], b[1], b[2], b[3]]))
        .collect())
}

/// Write 16 kHz mono PCM as a 16-bit WAV (used for in-app playback of formats the
/// system WebView can't play, e.g. ogg/opus on macOS).
pub fn write_wav_16k(path: &Path, samples: &[f32]) -> Result<()> {
    let mut f = std::io::BufWriter::new(File::create(path)?);
    let data_len = (samples.len() * 2) as u32;
    f.write_all(b"RIFF")?;
    f.write_all(&(36 + data_len).to_le_bytes())?;
    f.write_all(b"WAVEfmt ")?;
    f.write_all(&16u32.to_le_bytes())?;
    f.write_all(&1u16.to_le_bytes())?; // PCM
    f.write_all(&1u16.to_le_bytes())?; // mono
    f.write_all(&WHISPER_SAMPLE_RATE.to_le_bytes())?;
    f.write_all(&(WHISPER_SAMPLE_RATE * 2).to_le_bytes())?;
    f.write_all(&2u16.to_le_bytes())?;
    f.write_all(&16u16.to_le_bytes())?;
    f.write_all(b"data")?;
    f.write_all(&data_len.to_le_bytes())?;
    for s in samples {
        let v = (s.clamp(-1.0, 1.0) * 32767.0) as i16;
        f.write_all(&v.to_le_bytes())?;
    }
    f.flush()?;
    Ok(())
}

/// Whether the system WebView can be expected to play this format directly.
pub fn webview_can_play(ext: &str) -> bool {
    matches!(ext, "mp3" | "m4a" | "wav" | "mp4" | "mov" | "m4v" | "aac")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resample_length_is_correct() {
        let input: Vec<f32> = (0..44_100).map(|i| (i as f32 * 0.01).sin()).collect();
        let out = resample_to_16k(&input, 44_100).unwrap();
        assert_eq!(out.len(), 16_000);
        let out48 = resample_to_16k(&vec![0.1; 48_000 * 3], 48_000).unwrap();
        assert_eq!(out48.len(), 48_000);
    }
}
