// Typed wrappers around Tauri commands and events.
import { invoke, convertFileSrc } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

export type Quality = "fast" | "accurate";
export type ExportFormat = "docx" | "pdf" | "txt" | "srt";

export interface Language {
  code: string;
  name: string;
}
export interface AppInfo {
  version: string;
  languages: Language[];
  extensions: string[];
  diarization_available: boolean;
  data_dir: string;
}
export interface ModelStatus {
  quality: Quality;
  title: string;
  file: string;
  size: number;
  installed: boolean;
}
export interface Settings {
  save_dir: string;
  mirror: string;
  theme: "light" | "dark" | "system";
  language: string;
  quality: Quality;
  diarize: boolean;
  first_run_done: boolean;
}
export interface MediaProbe {
  name: string;
  format: string;
  duration_sec: number | null;
  size: number;
}
export interface RecentItem {
  id: string;
  name: string;
  format: string;
  created_at: number;
  duration_ms: number;
}
export interface Segment {
  start_ms: number;
  end_ms: number;
  text: string;
  speaker: number | null;
}
export interface Paragraph {
  id: number;
  start_ms: number;
  end_ms: number;
  speaker: number | null;
  text: string;
  segments: Segment[];
  edited: boolean;
}
export interface Transcript {
  language: string;
  duration_ms: number;
  speakers: string[];
  paragraphs: Paragraph[];
}
export interface Project {
  id: string;
  name: string;
  source_path: string;
  media_path: string;
  format: string;
  created_at: number;
  quality: Quality;
  transcript: Transcript;
}
export interface JobProgress {
  stage: "decoding" | "loading" | "transcribing" | "diarizing" | "saving";
  percent: number;
  eta_sec: number | null;
}
export interface JobSegment {
  start_ms: number;
  end_ms: number;
  text: string;
}
export interface ModelProgress {
  quality: Quality;
  index: number;
  count: number;
  downloaded: number;
  total: number;
  speed: number;
}
export interface FailInfo {
  message: string;
  cancelled: boolean;
}

export const api = {
  appInfo: () => invoke<AppInfo>("app_info"),
  listModels: () => invoke<ModelStatus[]>("list_models"),
  getSettings: () => invoke<Settings>("get_settings"),
  saveSettings: (settings: Settings) => invoke<void>("save_settings", { settings }),
  downloadModels: (qualities: Quality[]) => invoke<void>("download_models", { qualities }),
  cancelDownload: () => invoke<void>("cancel_download"),
  deleteModel: (quality: Quality) => invoke<void>("delete_model", { quality }),
  probeMedia: (path: string) => invoke<MediaProbe>("probe_media", { path }),
  startTranscription: (req: { path: string; language: string; quality: Quality; diarize: boolean }) =>
    invoke<void>("start_transcription", { req }),
  cancelTranscription: () => invoke<void>("cancel_transcription"),
  listRecent: () => invoke<RecentItem[]>("list_recent"),
  getProject: (id: string) => invoke<Project>("get_project", { id }),
  deleteProject: (id: string) => invoke<void>("delete_project", { id }),
  renameSpeaker: (id: string, index: number, name: string) =>
    invoke<Transcript>("rename_speaker", { id, index, name }),
  updateParagraph: (id: string, paragraphId: number, text: string) =>
    invoke<void>("update_paragraph", { id, paragraphId, text }),
  projectText: (id: string) => invoke<string>("project_text", { id }),
  defaultExportPath: (id: string, format: ExportFormat) =>
    invoke<string>("default_export_path", { id, format }),
  exportProject: (id: string, format: ExportFormat, path: string) =>
    invoke<string>("export_project", { id, format, path }),
};

export function on<T>(event: string, cb: (payload: T) => void): Promise<UnlistenFn> {
  return listen<T>(event, (e) => cb(e.payload));
}

export function mediaUrl(path: string): string {
  return convertFileSrc(path);
}
