//! Кнорозов PRO — Tauri backend: commands, background jobs, events.

mod storage;

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use knorozov_core::asr::Engine;
use knorozov_core::export::{DocMeta, ExportFormat};
use knorozov_core::models::{self, Quality};
use knorozov_core::pipeline::{self, JobOptions, Progress, Stage};
use knorozov_core::transcript::Transcript;
use knorozov_core::{audio, diarize, languages, timefmt};
use serde::{Deserialize, Serialize};
use storage::{Project, RecentItem, Settings, Store};
use tauri::{AppHandle, Emitter, Manager, State};

type CmdResult<T> = Result<T, String>;

fn err<E: std::fmt::Display>(e: E) -> String {
    format!("{e:#}")
}

#[derive(Default)]
struct Jobs {
    cancel_job: Arc<AtomicBool>,
    job_running: AtomicBool,
    cancel_download: Arc<AtomicBool>,
    download_running: AtomicBool,
    engine: Mutex<Option<(Quality, Arc<Engine>)>>,
}

struct AppState {
    store: Store,
    jobs: Arc<Jobs>,
}

// ---------- info / settings ----------

#[derive(Serialize)]
struct ModelStatus {
    quality: Quality,
    title: &'static str,
    file: &'static str,
    size: u64,
    installed: bool,
}

#[derive(Serialize)]
struct AppInfo {
    version: &'static str,
    languages: Vec<languages::Language>,
    extensions: &'static [&'static str],
    diarization_available: bool,
    data_dir: String,
}

#[tauri::command]
fn app_info(state: State<AppState>) -> AppInfo {
    AppInfo {
        version: env!("CARGO_PKG_VERSION"),
        languages: languages::all(),
        extensions: audio::SUPPORTED_EXTENSIONS,
        diarization_available: cfg!(feature = "diarization"),
        data_dir: state.store.root.display().to_string(),
    }
}

#[tauri::command]
fn list_models(state: State<AppState>) -> Vec<ModelStatus> {
    let dir = state.store.models_dir();
    models::CATALOG
        .iter()
        .filter(|m| m.quality != Quality::Tiny)
        .map(|m| ModelStatus {
            quality: m.quality,
            title: m.title,
            file: m.file,
            size: m.size,
            installed: models::is_installed(&dir, m.quality),
        })
        .collect()
}

fn default_save_dir(app: &AppHandle) -> String {
    app.path()
        .document_dir()
        .or_else(|_| app.path().home_dir())
        .map(|p| p.display().to_string())
        .unwrap_or_default()
}

#[tauri::command]
fn get_settings(app: AppHandle, state: State<AppState>) -> Settings {
    let mut s = state.store.load_settings();
    if s.save_dir.is_empty() || !Path::new(&s.save_dir).is_dir() {
        s.save_dir = default_save_dir(&app);
    }
    s
}

#[tauri::command]
fn save_settings(settings: Settings, state: State<AppState>) -> CmdResult<()> {
    state.store.save_settings(&settings).map_err(err)
}

// ---------- models ----------

#[derive(Clone, Serialize)]
struct ModelProgress {
    quality: Quality,
    index: usize,
    count: usize,
    downloaded: u64,
    total: u64,
    speed: f64,
}

#[derive(Clone, Serialize)]
struct ModelError {
    message: String,
    cancelled: bool,
}

#[tauri::command]
fn download_models(qualities: Vec<Quality>, app: AppHandle, state: State<AppState>) -> CmdResult<()> {
    let jobs = state.jobs.clone();
    if jobs.download_running.swap(true, Ordering::SeqCst) {
        return Err("Загрузка уже идёт".into());
    }
    jobs.cancel_download.store(false, Ordering::SeqCst);
    let dir = state.store.models_dir();
    let mirror = state.store.load_settings().mirror;
    std::thread::spawn(move || {
        let count = qualities.len();
        let mut result: Result<(), String> = Ok(());
        for (index, q) in qualities.into_iter().enumerate() {
            let app2 = app.clone();
            let r = models::download(&dir, q, &mirror, &jobs.cancel_download, move |p| {
                let _ = app2.emit(
                    "models:progress",
                    ModelProgress { quality: q, index, count, downloaded: p.downloaded, total: p.total, speed: p.speed },
                );
            });
            if let Err(e) = r {
                result = Err(err(e));
                break;
            }
        }
        jobs.download_running.store(false, Ordering::SeqCst);
        match result {
            Ok(()) => {
                let _ = app.emit("models:done", ());
            }
            Err(message) => {
                let cancelled = jobs.cancel_download.load(Ordering::SeqCst);
                let _ = app.emit("models:error", ModelError { message, cancelled });
            }
        }
    });
    Ok(())
}

#[tauri::command]
fn cancel_download(state: State<AppState>) {
    state.jobs.cancel_download.store(true, Ordering::SeqCst);
}

#[tauri::command]
fn delete_model(quality: Quality, state: State<AppState>) -> CmdResult<()> {
    let mut eng = state.jobs.engine.lock().unwrap();
    if matches!(&*eng, Some((q, _)) if *q == quality) {
        *eng = None;
    }
    let p = models::model_path(&state.store.models_dir(), quality);
    if p.exists() {
        std::fs::remove_file(p).map_err(err)?;
    }
    let part = state.store.models_dir().join(format!("{}.part", models::info(quality).file));
    std::fs::remove_file(part).ok();
    Ok(())
}

// ---------- media / transcription ----------

#[derive(Serialize)]
struct MediaProbe {
    name: String,
    format: String,
    duration_sec: Option<f64>,
    size: u64,
}

#[tauri::command]
fn probe_media(path: String) -> CmdResult<MediaProbe> {
    let p = PathBuf::from(&path);
    let size = std::fs::metadata(&p).map_err(err)?.len();
    let name = p.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
    match audio::probe(&p) {
        Ok(info) => Ok(MediaProbe { name, format: info.format, duration_sec: info.duration_sec, size }),
        Err(_) => Ok(MediaProbe {
            name,
            format: p.extension().map(|e| e.to_string_lossy().to_lowercase()).unwrap_or_default(),
            duration_sec: None,
            size,
        }),
    }
}

#[derive(Deserialize)]
struct StartRequest {
    path: String,
    language: String,
    quality: Quality,
    diarize: bool,
}

#[derive(Clone, Serialize)]
struct JobSegment {
    start_ms: u64,
    end_ms: u64,
    text: String,
}

#[derive(Clone, Serialize)]
struct JobError {
    message: String,
    cancelled: bool,
}

fn new_id() -> String {
    let now = chrono::Local::now();
    format!("{}-{:04x}", now.format("%Y%m%d-%H%M%S"), (now.timestamp_subsec_nanos() >> 8) & 0xffff)
}

fn embedding_model_path(app: &AppHandle) -> Option<PathBuf> {
    let rel = format!("models/{}", diarize::EMBEDDING_MODEL_FILE);
    if let Ok(p) = app.path().resolve(&rel, tauri::path::BaseDirectory::Resource) {
        if p.exists() {
            return Some(p);
        }
    }
    let dev = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("resources").join(&rel);
    dev.exists().then_some(dev)
}

#[tauri::command]
fn start_transcription(req: StartRequest, app: AppHandle, state: State<AppState>) -> CmdResult<()> {
    let jobs = state.jobs.clone();
    let model = models::model_path(&state.store.models_dir(), req.quality);
    if !models::is_installed(&state.store.models_dir(), req.quality) {
        return Err("Модель для этого режима ещё не скачана".into());
    }
    if jobs.job_running.swap(true, Ordering::SeqCst) {
        return Err("Расшифровка уже идёт".into());
    }
    jobs.cancel_job.store(false, Ordering::SeqCst);
    let playback_dir = state.store.playback_dir();
    let store_root = state.store.root.clone();
    let spk_model = embedding_model_path(&app);

    std::thread::spawn(move || {
        let cancel = jobs.cancel_job.clone();
        let emit_progress = {
            let app = app.clone();
            move |p: Progress| {
                let _ = app.emit("job:progress", p);
            }
        };
        let result = (|| -> anyhow::Result<String> {
            let src = PathBuf::from(&req.path);
            let ext = src.extension().map(|e| e.to_string_lossy().to_lowercase()).unwrap_or_default();
            emit_progress(Progress { stage: Stage::Decoding, percent: 0.0, eta_sec: None });
            let pcm = {
                let ep = emit_progress.clone();
                let c = cancel.clone();
                audio::decode_to_16k_mono(
                    &src,
                    move |f| ep(Progress { stage: Stage::Decoding, percent: 4.0 * f, eta_sec: None }),
                    move || c.load(Ordering::Relaxed),
                )?
            };
            if cancel.load(Ordering::Relaxed) {
                anyhow::bail!("отменено");
            }
            emit_progress(Progress { stage: Stage::Loading, percent: 4.5, eta_sec: None });
            let engine = {
                let mut slot = jobs.engine.lock().unwrap();
                match &*slot {
                    Some((q, e)) if *q == req.quality => e.clone(),
                    _ => {
                        *slot = None; // free the previous model first
                        let e = Arc::new(Engine::load(&model)?);
                        *slot = Some((req.quality, e.clone()));
                        e
                    }
                }
            };
            let opts = JobOptions {
                language: req.language.clone(),
                diarize: req.diarize && spk_model.is_some(),
                embedding_model: spk_model.clone(),
                threads: None,
            };
            let app_seg = app.clone();
            let (transcript, stats) = pipeline::run(
                &engine,
                &pcm,
                &opts,
                emit_progress.clone(),
                move |s| {
                    let _ = app_seg.emit("job:segment", JobSegment { start_ms: s.start_ms, end_ms: s.end_ms, text: s.text });
                },
                cancel.clone(),
            )?;
            log::info!("transcribed {:.1}s audio, RTF {:.3}", stats.audio_sec, stats.rtf);
            emit_progress(Progress { stage: Stage::Saving, percent: 99.5, eta_sec: Some(0.0) });

            let id = new_id();
            let media_path = if audio::webview_can_play(&ext) {
                src.clone()
            } else {
                let wav = playback_dir.join(format!("{id}.wav"));
                audio::write_wav_16k(&wav, &pcm)?;
                wav
            };
            let project = Project {
                id: id.clone(),
                name: src.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default(),
                source_path: src.display().to_string(),
                media_path: media_path.display().to_string(),
                format: ext,
                created_at: chrono::Utc::now().timestamp_millis(),
                quality: req.quality,
                transcript,
            };
            Store { root: store_root.clone() }.save_project(&project)?;
            Ok(id)
        })();
        jobs.job_running.store(false, Ordering::SeqCst);
        match result {
            Ok(id) => {
                let _ = app.emit("job:done", id);
            }
            Err(e) => {
                let cancelled = cancel.load(Ordering::Relaxed);
                let _ = app.emit("job:error", JobError { message: err(e), cancelled });
            }
        }
    });
    Ok(())
}

#[tauri::command]
fn cancel_transcription(state: State<AppState>) {
    state.jobs.cancel_job.store(true, Ordering::SeqCst);
}

// ---------- projects ----------

#[tauri::command]
fn list_recent(state: State<AppState>) -> Vec<RecentItem> {
    state.store.recent()
}

#[tauri::command]
fn get_project(id: String, state: State<AppState>) -> CmdResult<Project> {
    state.store.load_project(&id).map_err(err)
}

#[tauri::command]
fn delete_project(id: String, state: State<AppState>) -> CmdResult<()> {
    state.store.delete_project(&id).map_err(err)
}

#[tauri::command]
fn rename_speaker(id: String, index: usize, name: String, state: State<AppState>) -> CmdResult<Transcript> {
    let mut p = state.store.load_project(&id).map_err(err)?;
    if !p.transcript.rename_speaker(index, &name) {
        return Err("Некорректное имя спикера".into());
    }
    state.store.update_project(&p).map_err(err)?;
    Ok(p.transcript)
}

#[tauri::command]
fn update_paragraph(id: String, paragraph_id: u32, text: String, state: State<AppState>) -> CmdResult<()> {
    let mut p = state.store.load_project(&id).map_err(err)?;
    p.transcript.set_paragraph_text(paragraph_id, &text);
    state.store.update_project(&p).map_err(err)
}

#[tauri::command]
fn project_text(id: String, state: State<AppState>) -> CmdResult<String> {
    let p = state.store.load_project(&id).map_err(err)?;
    Ok(p.transcript.plain_text(p.transcript.has_speakers(), false))
}

fn doc_meta(p: &Project) -> DocMeta {
    let date = chrono::DateTime::from_timestamp_millis(p.created_at)
        .map(|d| d.with_timezone(&chrono::Local).format("%d.%m.%Y").to_string())
        .unwrap_or_default();
    let mut parts = vec![date, timefmt::duration(p.transcript.duration_ms)];
    if let Some(l) = languages::name_of(&p.transcript.language) {
        parts.push(l.to_lowercase());
    }
    if p.transcript.has_speakers() {
        parts.push(format!("спикеров: {}", p.transcript.speakers.len()));
    }
    DocMeta { title: p.name.clone(), subtitle: parts.join(" · ") }
}

#[tauri::command]
fn default_export_path(id: String, format: ExportFormat, app: AppHandle, state: State<AppState>) -> CmdResult<String> {
    let p = state.store.load_project(&id).map_err(err)?;
    let mut dir = state.store.load_settings().save_dir;
    if dir.is_empty() || !Path::new(&dir).is_dir() {
        dir = default_save_dir(&app);
    }
    let stem = Path::new(&p.name).file_stem().map(|s| s.to_string_lossy().into_owned()).unwrap_or("Расшифровка".into());
    Ok(Path::new(&dir).join(format!("{stem}.{}", format.extension())).display().to_string())
}

#[tauri::command]
fn export_project(id: String, format: ExportFormat, path: String, state: State<AppState>) -> CmdResult<String> {
    let p = state.store.load_project(&id).map_err(err)?;
    let mut out = PathBuf::from(&path);
    if out.extension().is_none() {
        out.set_extension(format.extension());
    }
    let bytes = knorozov_core::export::export_bytes(&p.transcript, &doc_meta(&p), format).map_err(err)?;
    std::fs::write(&out, bytes).map_err(err)?;
    Ok(out.display().to_string())
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let root = app.path().app_data_dir()?;
            let store = Store::new(root)?;
            app.manage(AppState { store, jobs: Arc::new(Jobs::default()) });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            app_info,
            list_models,
            get_settings,
            save_settings,
            download_models,
            cancel_download,
            delete_model,
            probe_media,
            start_transcription,
            cancel_transcription,
            list_recent,
            get_project,
            delete_project,
            rename_speaker,
            update_paragraph,
            project_text,
            default_export_path,
            export_project,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Кнорозов PRO");
}
