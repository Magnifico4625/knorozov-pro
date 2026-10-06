//! Model catalog and resumable, checksum-verified downloads from Hugging Face (or a mirror).

use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use anyhow::{anyhow, bail, Context, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const DEFAULT_MIRROR: &str = "https://huggingface.co";
pub const HF_REPO: &str = "ggerganov/whisper.cpp";

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Quality {
    /// «Быстро»
    Fast,
    /// «Точно»
    Accurate,
    /// Tests / CI only.
    Tiny,
}

#[derive(Debug, Clone, Serialize)]
pub struct ModelInfo {
    pub quality: Quality,
    pub title: &'static str,
    pub file: &'static str,
    pub size: u64,
    pub sha256: &'static str,
    pub license: &'static str,
}

pub static CATALOG: &[ModelInfo] = &[
    ModelInfo {
        quality: Quality::Fast,
        title: "Быстро — Whisper small (q5_1)",
        file: "ggml-small-q5_1.bin",
        size: 190_085_487,
        sha256: "ae85e4a935d7a567bd102fe55afc16bb595bdb618e11b2fc7591bc08120411bb",
        license: "MIT",
    },
    ModelInfo {
        quality: Quality::Accurate,
        title: "Точно — Whisper large-v3-turbo (q5_0)",
        file: "ggml-large-v3-turbo-q5_0.bin",
        size: 574_041_195,
        sha256: "394221709cd5ad1f40c46e6031ca61bce88931e6e088c188294c6d5a55ffa7e2",
        license: "MIT",
    },
    ModelInfo {
        quality: Quality::Tiny,
        title: "Тест — Whisper tiny",
        file: "ggml-tiny.bin",
        size: 77_691_713,
        sha256: "be07e048e1e599ad46341c8d2a135645097a538221678b7acdd1b1919c6e1b21",
        license: "MIT",
    },
];

pub fn info(q: Quality) -> &'static ModelInfo {
    CATALOG.iter().find(|m| m.quality == q).expect("model in catalog")
}

/// Build the download URL. `mirror` is either a host root (`https://hf-mirror.com`) in which case
/// the HF path layout is appended, or a full template containing `{file}`.
pub fn model_url(mirror: &str, file: &str) -> String {
    let m = mirror.trim();
    let m = if m.is_empty() { DEFAULT_MIRROR } else { m };
    if m.contains("{file}") {
        m.replace("{file}", file)
    } else {
        format!("{}/{}/resolve/main/{}", m.trim_end_matches('/'), HF_REPO, file)
    }
}

pub fn model_path(dir: &Path, q: Quality) -> PathBuf {
    dir.join(info(q).file)
}

/// A model counts as installed if the file exists with the expected size
/// (the SHA-256 is verified right after download).
pub fn is_installed(dir: &Path, q: Quality) -> bool {
    fs::metadata(model_path(dir, q)).map(|m| m.len() == info(q).size).unwrap_or(false)
}

#[derive(Debug, Clone, Serialize)]
pub struct DownloadProgress {
    pub downloaded: u64,
    pub total: u64,
    /// bytes per second (smoothed)
    pub speed: f64,
}

pub fn sha256_file(path: &Path) -> Result<String> {
    let mut f = File::open(path)?;
    let mut h = Sha256::new();
    let mut buf = vec![0u8; 1 << 20];
    loop {
        let n = f.read(&mut buf)?;
        if n == 0 {
            break;
        }
        h.update(&buf[..n]);
    }
    Ok(hex::encode(h.finalize()))
}

/// Download `q` into `dir`, resuming a previous `.part` file if present.
pub fn download(
    dir: &Path,
    q: Quality,
    mirror: &str,
    cancel: &AtomicBool,
    mut on_progress: impl FnMut(DownloadProgress),
) -> Result<PathBuf> {
    let m = info(q);
    download_file(&model_url(mirror, m.file), dir, m.file, m.size, m.sha256, cancel, &mut on_progress)
}

#[allow(clippy::too_many_arguments)]
pub fn download_file(
    url: &str,
    dir: &Path,
    file: &str,
    size: u64,
    sha256: &str,
    cancel: &AtomicBool,
    on_progress: &mut dyn FnMut(DownloadProgress),
) -> Result<PathBuf> {
    fs::create_dir_all(dir).with_context(|| format!("не удалось создать папку {}", dir.display()))?;
    let final_path = dir.join(file);
    if fs::metadata(&final_path).map(|m| m.len() == size).unwrap_or(false) {
        return Ok(final_path);
    }
    let part = dir.join(format!("{file}.part"));
    let mut have = fs::metadata(&part).map(|m| m.len()).unwrap_or(0);
    if have > size {
        fs::remove_file(&part).ok();
        have = 0;
    }

    if have < size {
        let agent: ureq::Agent = ureq::Agent::config_builder()
            .timeout_connect(Some(Duration::from_secs(20)))
            .timeout_recv_response(Some(Duration::from_secs(60)))
            .timeout_recv_body(None)
            .user_agent("KnorozovPRO/0.1")
            .build()
            .into();
        let mut req = agent.get(url);
        if have > 0 {
            req = req.header("Range", format!("bytes={have}-"));
        }
        let resp = req.call().map_err(|e| anyhow!("не удалось подключиться к {url}: {e}"))?;
        let status = resp.status().as_u16();
        let mut out = if status == 206 {
            OpenOptions::new().append(true).open(&part)?
        } else if status == 200 {
            have = 0;
            File::create(&part)?
        } else {
            bail!("сервер ответил {status}");
        };
        let mut reader = resp.into_body().into_reader();
        let mut buf = vec![0u8; 256 * 1024];
        let mut downloaded = have;
        let started = Instant::now();
        let start_bytes = have;
        let mut last = Instant::now() - Duration::from_secs(1);
        loop {
            if cancel.load(Ordering::Relaxed) {
                out.flush().ok();
                bail!("загрузка отменена");
            }
            let n = reader.read(&mut buf).map_err(|e| anyhow!("обрыв соединения: {e}"))?;
            if n == 0 {
                break;
            }
            out.write_all(&buf[..n])?;
            downloaded += n as u64;
            if last.elapsed() >= Duration::from_millis(200) {
                last = Instant::now();
                let secs = started.elapsed().as_secs_f64().max(0.001);
                on_progress(DownloadProgress {
                    downloaded,
                    total: size,
                    speed: (downloaded - start_bytes) as f64 / secs,
                });
            }
        }
        out.flush()?;
        drop(out);
        if downloaded != size {
            bail!("загружено {downloaded} из {size} байт — попробуйте ещё раз (загрузка продолжится)");
        }
    }
    on_progress(DownloadProgress { downloaded: size, total: size, speed: 0.0 });
    let got = sha256_file(&part)?;
    if !got.eq_ignore_ascii_case(sha256) {
        fs::remove_file(&part).ok();
        bail!("контрольная сумма не совпала — файл повреждён, скачайте заново");
    }
    fs::rename(&part, &final_path)?;
    Ok(final_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn urls() {
        assert_eq!(
            model_url("", "ggml-tiny.bin"),
            "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-tiny.bin"
        );
        assert_eq!(
            model_url("https://hf-mirror.com/", "a.bin"),
            "https://hf-mirror.com/ggerganov/whisper.cpp/resolve/main/a.bin"
        );
        assert_eq!(model_url("http://10.0.0.5/models/{file}", "a.bin"), "http://10.0.0.5/models/a.bin");
    }

    #[test]
    fn catalog_sane() {
        for m in CATALOG {
            assert_eq!(m.sha256.len(), 64);
            assert!(m.size > 10_000_000);
        }
        assert_eq!(info(Quality::Fast).file, "ggml-small-q5_1.bin");
    }
}
