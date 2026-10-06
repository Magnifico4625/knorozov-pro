<script lang="ts">
  import { onMount } from "svelte";
  import { getCurrentWebview } from "@tauri-apps/api/webview";
  import { open } from "@tauri-apps/plugin-dialog";
  import Icon from "../components/Icon.svelte";
  import { api } from "../lib/api";
  import { app, modelInstalled, openProject, refreshRecent, saveSettings, startDownload, startJob, toast } from "../lib/state.svelte";
  import { date, duration } from "../lib/format";
  import emptyArt from "../assets/empty-state.png";

  let dragging = $state(false);
  let selected = $state<string | null>(null);
  let selectedName = $derived(selected ? (selected.split(/[\\/]/).pop() ?? selected) : "");
  let confirmDelete = $state<string | null>(null);

  const exts = $derived(app.info?.extensions ?? []);

  function accept(path: string): boolean {
    const ext = path.split(".").pop()?.toLowerCase() ?? "";
    if (!exts.includes(ext)) {
      toast(`Формат .${ext} не поддерживается`, "error");
      return false;
    }
    return true;
  }

  onMount(() => {
    let un: (() => void) | undefined;
    getCurrentWebview()
      .onDragDropEvent((e) => {
        const p = e.payload;
        if (p.type === "enter" || p.type === "over") dragging = true;
        else if (p.type === "leave") dragging = false;
        else if (p.type === "drop") {
          dragging = false;
          const path = p.paths[0];
          if (path && accept(path)) selected = path;
        }
      })
      .then((f) => (un = f));
    return () => un?.();
  });

  async function pick() {
    const res = await open({
      multiple: false,
      directory: false,
      filters: [{ name: "Аудио и видео", extensions: exts }],
    });
    if (typeof res === "string" && accept(res)) selected = res;
  }

  async function go() {
    if (!selected || !app.settings) return;
    await saveSettings();
    if (!modelInstalled(app.settings.quality)) {
      toast("Сначала нужно скачать модель для этого режима");
      app.settingsSection = "models";
      app.settingsOpen = true;
      startDownload([app.settings.quality]);
      return;
    }
    startJob(selected);
  }

  async function remove(id: string) {
    await api.deleteProject(id);
    confirmDelete = null;
    await refreshRecent();
  }

  function iconFor(fmt: string) {
    if (["mp4", "mov", "m4v", "mkv", "webm"].includes(fmt)) return "video";
    if (["ogg", "opus", "oga"].includes(fmt)) return "mic";
    return "audio";
  }
</script>

<div class="grid">
  <section class="card left">
    <button
      class="drop"
      class:dragging
      class:has={!!selected}
      onclick={pick}
      aria-label="Выбрать файл"
    >
      <img class="illustration" src={emptyArt} alt="" draggable="false" />
      {#if selected}
        <div class="title">{selectedName}</div>
        <div class="muted small">Файл выбран. Нажмите «Расшифровать» или выберите другой.</div>
      {:else}
        <div class="title">Перетащите аудио или видео сюда</div>
      {/if}
      <span class="btn primary pick"><Icon name="file" size={16} /> Выбрать файл</span>
      <div class="formats faint">mp3, m4a, wav, ogg (голосовые), mp4, mov</div>
    </button>

    {#if app.settings}
      <div class="form">
        <label class="row" for="lang">
          <span class="label">Язык:</span>
          <select id="lang" class="select" bind:value={app.settings.language} onchange={saveSettings}>
            <option value="auto">Определить автоматически</option>
            {#each app.info?.languages ?? [] as l (l.code)}
              <option value={l.code}>{l.name}</option>
            {/each}
          </select>
        </label>
        <div class="row">
          <span class="label">Качество:</span>
          <div class="segmented quality">
            <button class:on={app.settings.quality === "fast"} onclick={() => ((app.settings!.quality = "fast"), saveSettings())}>Быстро</button>
            <button class:on={app.settings.quality === "accurate"} onclick={() => ((app.settings!.quality = "accurate"), saveSettings())}>Точно</button>
          </div>
          {#if !modelInstalled(app.settings.quality)}
            <span class="warn">модель не скачана</span>
          {/if}
        </div>
        <div class="row">
          <span class="label"></span>
          <label class="checkbox">
            <input type="checkbox" bind:checked={app.settings.diarize} onchange={saveSettings} disabled={!app.info?.diarization_available} />
            Разделить по спикерам
          </label>
        </div>
      </div>
    {/if}

    <button class="btn primary big go" disabled={!selected} onclick={go}>Расшифровать</button>
    <div class="center">
      <span class="badge-offline"><Icon name="shield" size={16} /> Работает офлайн, ваши записи никуда не отправляются</span>
    </div>
  </section>

  <section class="card right">
    <h2>Недавние</h2>
    {#if app.recent.length === 0}
      <div class="empty">
        <p class="muted">Здесь появятся ваши расшифровки.<br />Они хранятся только на этом компьютере.</p>
      </div>
    {:else}
      <div class="table">
        <div class="thead">
          <span>Название</span><span>Дата</span><span class="r">Длительность</span><span></span>
        </div>
        <div class="tbody">
          {#each app.recent as r (r.id)}
            <div class="tr" role="button" tabindex="0" onclick={() => openProject(r.id)} onkeydown={(e) => e.key === "Enter" && openProject(r.id)}>
              <span class="name"><span class="ficon"><Icon name={iconFor(r.format)} size={15} /></span><span class="ellipsis">{r.name}</span></span>
              <span class="muted">{date(r.created_at)}</span>
              <span class="muted r">{duration(r.duration_ms)}</span>
              <span class="r">
                {#if confirmDelete === r.id}
                  <button class="btn ghost small danger" onclick={(e) => (e.stopPropagation(), remove(r.id))}>Удалить?</button>
                {:else}
                  <button class="btn ghost icon del" title="Удалить" onclick={(e) => (e.stopPropagation(), (confirmDelete = r.id))}><Icon name="trash" size={15} /></button>
                {/if}
              </span>
            </div>
          {/each}
        </div>
      </div>
    {/if}
  </section>
</div>

<style>
  .grid {
    height: 100%;
    display: grid;
    grid-template-columns: minmax(420px, 1.05fr) minmax(360px, 1fr);
    gap: 18px;
  }
  .left {
    padding: 18px;
    display: flex;
    flex-direction: column;
    gap: 14px;
    min-height: 0;
    overflow: auto;
  }
  .drop {
    flex: 1;
    min-height: 220px;
    border: 1.5px dashed color-mix(in srgb, var(--primary) 45%, var(--border-strong));
    border-radius: 14px;
    background: color-mix(in srgb, var(--primary) 4%, var(--card));
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 10px;
    padding: 16px;
    cursor: pointer;
    transition: background 0.15s, border-color 0.15s;
  }
  .drop:hover,
  .drop.dragging {
    background: color-mix(in srgb, var(--primary) 9%, var(--card));
    border-color: var(--primary);
  }
  .drop img {
    width: min(260px, 70%);
    height: auto;
    pointer-events: none;
  }
  .title {
    font-size: 17px;
    font-weight: 600;
    text-align: center;
    word-break: break-word;
  }
  .small {
    font-size: 12.5px;
  }
  .pick {
    margin-top: 4px;
    min-width: 200px;
  }
  .formats {
    font-size: 12.5px;
  }
  .form {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }
  .row {
    display: flex;
    align-items: center;
    gap: 12px;
  }
  .label {
    width: 72px;
    flex: none;
    color: var(--text-2);
  }
  .row .select {
    width: 260px;
  }
  .quality {
    width: 260px;
  }
  .warn {
    font-size: 12px;
    color: #b45309;
  }
  .go {
    width: 100%;
  }
  .center {
    display: flex;
    justify-content: center;
  }
  .right {
    padding: 18px 18px 8px;
    display: flex;
    flex-direction: column;
    min-height: 0;
  }
  h2 {
    font-size: 16px;
    margin: 2px 0 12px;
  }
  .empty {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    text-align: center;
  }
  .table {
    display: flex;
    flex-direction: column;
    min-height: 0;
    flex: 1;
  }
  .thead,
  .tr {
    display: grid;
    grid-template-columns: 1fr 96px 100px 44px;
    align-items: center;
    gap: 8px;
    padding: 0 8px;
  }
  .thead {
    font-size: 12.5px;
    color: var(--muted);
    height: 34px;
    background: var(--card-2);
    border-radius: 8px;
  }
  .tbody {
    overflow: auto;
    min-height: 0;
  }
  .tr {
    height: 46px;
    border-bottom: 1px solid var(--border);
    cursor: pointer;
    border-radius: 6px;
  }
  .tr:hover {
    background: var(--card-2);
  }
  .tr .del {
    opacity: 0;
  }
  .tr:hover .del {
    opacity: 1;
  }
  .name {
    display: flex;
    align-items: center;
    gap: 10px;
    min-width: 0;
  }
  .ficon {
    color: var(--muted);
    display: flex;
  }
  .ellipsis {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .r {
    text-align: right;
    justify-self: end;
  }
  .btn.small {
    height: 28px;
    padding: 0 8px;
    font-size: 12.5px;
  }
</style>
