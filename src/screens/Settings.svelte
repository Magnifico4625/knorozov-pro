<script lang="ts">
  import { onMount, tick } from "svelte";
  import { open } from "@tauri-apps/plugin-dialog";
  import Icon from "../components/Icon.svelte";
  import { api, type Quality } from "../lib/api";
  import { app, applyTheme, refreshModels, saveSettings, startDownload, toast } from "../lib/state.svelte";
  import { mb, speed } from "../lib/format";

  let mirrorDraft = $state(app.settings?.mirror ?? "");
  const presets = [
    { url: "https://huggingface.co", label: "Hugging Face (основной)" },
    { url: "https://hf-mirror.com", label: "hf-mirror.com (зеркало)" },
  ];

  onMount(async () => {
    await tick();
    if (app.settingsSection) document.getElementById(`sec-${app.settingsSection}`)?.scrollIntoView({ block: "start" });
  });

  function close() {
    app.settingsOpen = false;
  }
  async function pickFolder() {
    const dir = await open({ directory: true, multiple: false, defaultPath: app.settings?.save_dir });
    if (typeof dir === "string" && app.settings) {
      app.settings.save_dir = dir;
      await saveSettings();
    }
  }
  async function setTheme(t: "light" | "dark" | "system") {
    if (!app.settings) return;
    app.settings.theme = t;
    applyTheme();
    await saveSettings();
  }
  async function saveMirror() {
    if (!app.settings) return;
    const v = mirrorDraft.trim();
    if (v && !/^https?:\/\//.test(v)) {
      toast("Адрес зеркала должен начинаться с http:// или https://", "error");
      return;
    }
    app.settings.mirror = v || "https://huggingface.co";
    mirrorDraft = app.settings.mirror;
    await saveSettings();
    toast("Зеркало сохранено");
  }
  async function del(q: Quality) {
    await api.deleteModel(q);
    await refreshModels();
  }
  function pct(q: Quality) {
    const p = app.download.progress;
    if (!p || p.quality !== q || !p.total) return null;
    return Math.round((p.downloaded / p.total) * 100);
  }
</script>

<div class="backdrop" role="presentation" onclick={close}></div>
<div class="panel card" role="dialog" aria-label="Настройки">
  <div class="head">
    <h2><Icon name="gear" size={18} /> Настройки</h2>
    <button class="btn ghost icon" onclick={close} title="Закрыть"><Icon name="x" /></button>
  </div>

  <div class="scroll">
    {#if app.settings}
      <section id="sec-folder">
        <h3><Icon name="folder" size={16} /> Папка для сохранения</h3>
        <p class="muted small">Сюда по умолчанию сохраняются экспортированные файлы (Word, PDF, TXT, SRT).</p>
        <div class="pathrow">
          <div class="path" title={app.settings.save_dir}>{app.settings.save_dir || "—"}</div>
          <button class="btn" onclick={pickFolder}>Изменить…</button>
        </div>
      </section>

      <section id="sec-models">
        <h3><Icon name="download" size={16} /> Модели распознавания</h3>
        <p class="muted small">Модели скачиваются один раз и дальше работают без интернета.</p>
        {#each app.models as m (m.quality)}
          <div class="model">
            <div class="minfo">
              <div class="mtitle">{m.title}</div>
              <div class="muted small">{mb(m.size)} · {m.installed ? "скачана" : "не скачана"}</div>
              {#if pct(m.quality) !== null && app.download.running}
                <div class="mprog">
                  <div class="progress"><div style="width: {pct(m.quality)}%"></div></div>
                  <span class="muted small">{pct(m.quality)}% · {speed(app.download.progress?.speed ?? 0)}</span>
                </div>
              {/if}
            </div>
            {#if m.installed}
              <span class="ok"><Icon name="check" size={16} /></span>
              <button class="btn ghost danger" onclick={() => del(m.quality)} disabled={app.download.running}>Удалить</button>
            {:else if app.download.running && app.download.queue.includes(m.quality)}
              <button class="btn" onclick={() => api.cancelDownload()}>Отмена</button>
            {:else}
              <button class="btn primary" onclick={() => startDownload([m.quality])} disabled={app.download.running}>Скачать</button>
            {/if}
          </div>
        {/each}
        {#if app.download.error}
          <p class="err small">{app.download.error}</p>
        {/if}

        <label class="small label" for="mirror">Адрес для загрузки моделей (зеркало)</label>
        <div class="pathrow">
          <input id="mirror" class="input grow" bind:value={mirrorDraft} placeholder="https://huggingface.co" />
          <button class="btn" onclick={saveMirror}>Сохранить</button>
        </div>
        <div class="presets">
          {#each presets as p (p.url)}
            <button class="chip" class:on={app.settings.mirror === p.url} onclick={() => ((mirrorDraft = p.url), saveMirror())}>{p.label}</button>
          {/each}
        </div>
        <p class="faint small">
          Если Hugging Face недоступен, укажите зеркало. Можно указать и свой сервер: адрес с <code>{"{file}"}</code>,
          например <code>http://192.168.1.10/models/{"{file}"}</code>.
        </p>
      </section>

      <section id="sec-theme">
        <h3><Icon name="theme" size={16} /> Тема</h3>
        <div class="segmented">
          <button class:on={app.settings.theme === "light"} onclick={() => setTheme("light")}>Светлая</button>
          <button class:on={app.settings.theme === "dark"} onclick={() => setTheme("dark")}>Тёмная</button>
          <button class:on={app.settings.theme === "system"} onclick={() => setTheme("system")}>Как в системе</button>
        </div>
      </section>

      <section>
        <h3><Icon name="info" size={16} /> О программе</h3>
        <p class="muted small">Кнорозов PRO {app.info?.version} — офлайн-расшифровка аудио и видео.</p>
        <button class="btn" onclick={() => (app.aboutOpen = true)}>О программе и лицензии</button>
      </section>
    {/if}
  </div>
</div>

<style>
  .backdrop {
    position: fixed;
    inset: 0;
    background: rgba(15, 23, 42, 0.25);
    z-index: 40;
    animation: fade 0.15s;
  }
  .panel {
    position: fixed;
    top: 12px;
    right: 12px;
    bottom: 12px;
    width: 460px;
    z-index: 41;
    display: flex;
    flex-direction: column;
    box-shadow: var(--shadow-lg);
    animation: slide 0.2s ease-out;
  }
  @keyframes fade {
    from {
      opacity: 0;
    }
  }
  @keyframes slide {
    from {
      transform: translateX(30px);
      opacity: 0;
    }
  }
  .head {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 14px 14px 8px 20px;
  }
  h2 {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 17px;
    margin: 0;
  }
  .scroll {
    overflow: auto;
    padding: 0 20px 20px;
  }
  section {
    padding: 16px 0;
    border-bottom: 1px solid var(--border);
  }
  section:last-child {
    border-bottom: 0;
  }
  h3 {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 14.5px;
    margin: 0 0 6px;
  }
  .small {
    font-size: 12.5px;
  }
  .pathrow {
    display: flex;
    gap: 8px;
    align-items: center;
    margin-top: 8px;
  }
  .path {
    flex: 1;
    min-width: 0;
    padding: 8px 10px;
    background: var(--card-2);
    border: 1px solid var(--border);
    border-radius: 8px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    font-size: 13px;
    direction: rtl;
    text-align: left;
  }
  .grow {
    flex: 1;
    min-width: 0;
  }
  .model {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 10px 0;
  }
  .minfo {
    flex: 1;
    min-width: 0;
  }
  .mtitle {
    font-weight: 500;
  }
  .mprog {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-top: 6px;
  }
  .mprog .progress {
    flex: 1;
  }
  .ok {
    color: var(--green-text);
    display: flex;
  }
  .err {
    color: var(--danger);
    user-select: text;
    -webkit-user-select: text;
  }
  .label {
    display: block;
    margin-top: 12px;
    color: var(--text-2);
  }
  .presets {
    display: flex;
    gap: 6px;
    flex-wrap: wrap;
    margin-top: 8px;
  }
  .chip {
    border: 1px solid var(--border-strong);
    background: var(--card);
    border-radius: 99px;
    padding: 4px 10px;
    font-size: 12.5px;
    cursor: pointer;
  }
  .chip.on {
    border-color: var(--primary);
    color: var(--primary-text);
    background: var(--primary-soft);
  }
  code {
    font-size: 11.5px;
    background: var(--card-2);
    padding: 1px 4px;
    border-radius: 4px;
    user-select: text;
    -webkit-user-select: text;
  }
</style>
