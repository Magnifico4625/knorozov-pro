// Global UI state (Svelte 5 runes).
import {
  api,
  on,
  type AppInfo,
  type JobProgress,
  type JobSegment,
  type MediaProbe,
  type ModelProgress,
  type ModelStatus,
  type Quality,
  type RecentItem,
  type Settings,
  type FailInfo,
} from "./api";

export type Screen = "loading" | "firstrun" | "main" | "processing" | "editor";

export const app = $state({
  screen: "loading" as Screen,
  info: null as AppInfo | null,
  settings: null as Settings | null,
  models: [] as ModelStatus[],
  recent: [] as RecentItem[],
  settingsOpen: false,
  settingsSection: "" as string,
  aboutOpen: false,
  toast: "" as string,
  toastKind: "info" as "info" | "error",
  // processing
  file: null as (MediaProbe & { path: string }) | null,
  progress: null as JobProgress | null,
  segments: [] as JobSegment[],
  // editor
  projectId: "" as string,
  // downloads
  download: {
    running: false,
    progress: null as ModelProgress | null,
    error: "" as string,
    queue: [] as Quality[],
  },
});

let toastTimer: ReturnType<typeof setTimeout> | undefined;
export function toast(msg: string, kind: "info" | "error" = "info") {
  app.toast = msg;
  app.toastKind = kind;
  clearTimeout(toastTimer);
  toastTimer = setTimeout(() => (app.toast = ""), kind === "error" ? 6000 : 2500);
}

export async function refreshModels() {
  app.models = await api.listModels();
}
export async function refreshRecent() {
  app.recent = await api.listRecent();
}
export function modelInstalled(q: Quality): boolean {
  return app.models.find((m) => m.quality === q)?.installed ?? false;
}

export async function saveSettings() {
  if (app.settings) await api.saveSettings($state.snapshot(app.settings));
}

export function applyTheme() {
  const t = app.settings?.theme ?? "light";
  const dark = t === "dark" || (t === "system" && window.matchMedia("(prefers-color-scheme: dark)").matches);
  document.documentElement.dataset.theme = dark ? "dark" : "light";
}

export async function startDownload(qualities: Quality[]) {
  const missing = qualities.filter((q) => !modelInstalled(q));
  if (!missing.length) return;
  app.download.error = "";
  app.download.queue = missing;
  app.download.running = true;
  app.download.progress = null;
  try {
    await api.downloadModels(missing);
  } catch (e) {
    app.download.running = false;
    app.download.error = String(e);
  }
}

export async function startJob(path: string) {
  const s = app.settings!;
  try {
    const probe = await api.probeMedia(path);
    app.file = { ...probe, path };
    app.progress = { stage: "decoding", percent: 0, eta_sec: null };
    app.segments = [];
    app.screen = "processing";
    await api.startTranscription({ path, language: s.language, quality: s.quality, diarize: s.diarize });
  } catch (e) {
    app.screen = "main";
    toast(String(e), "error");
  }
}

export async function openProject(id: string) {
  app.projectId = id;
  app.screen = "editor";
}

export async function init() {
  const [info, settings] = await Promise.all([api.appInfo(), api.getSettings()]);
  app.info = info;
  app.settings = settings;
  applyTheme();
  window.matchMedia("(prefers-color-scheme: dark)").addEventListener("change", applyTheme);
  await Promise.all([refreshModels(), refreshRecent()]);

  on<ModelProgress>("models:progress", (p) => {
    app.download.running = true;
    app.download.progress = p;
  });
  on<null>("models:done", async () => {
    app.download.running = false;
    app.download.progress = null;
    await refreshModels();
    if (app.settings && !app.settings.first_run_done) {
      app.settings.first_run_done = true;
      await saveSettings();
    }
    if (app.screen === "firstrun") app.screen = "main";
    toast("Модели загружены — можно работать офлайн");
  });
  on<FailInfo>("models:error", async (e) => {
    app.download.running = false;
    app.download.error = e.cancelled ? "" : e.message;
    await refreshModels();
    if (e.cancelled) toast("Загрузка остановлена. Её можно продолжить позже.");
  });
  on<JobProgress>("job:progress", (p) => (app.progress = p));
  on<JobSegment>("job:segment", (s) => {
    if (s.text.trim()) app.segments.push(s);
  });
  on<string>("job:done", async (id) => {
    await refreshRecent();
    openProject(id);
  });
  on<FailInfo>("job:error", (e) => {
    app.screen = "main";
    if (!e.cancelled) toast(e.message, "error");
  });

  const anyModel = app.models.some((m) => m.installed);
  app.screen = anyModel ? "main" : "firstrun";
}
