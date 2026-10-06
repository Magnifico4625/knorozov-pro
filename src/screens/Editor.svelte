<script lang="ts">
  import { onMount, tick } from "svelte";
  import { save } from "@tauri-apps/plugin-dialog";
  import { writeText } from "@tauri-apps/plugin-clipboard-manager";
  import { revealItemInDir } from "@tauri-apps/plugin-opener";
  import Icon from "../components/Icon.svelte";
  import Player from "../components/Player.svelte";
  import { api, mediaUrl, type ExportFormat, type Paragraph, type Project } from "../lib/api";
  import { app, toast } from "../lib/state.svelte";
  import { chip, date, duration } from "../lib/format";

  let project = $state<Project | null>(null);
  let loadError = $state("");
  let query = $state("");
  let matchIndex = $state(0);
  let exportOpen = $state(false);
  let menuFor = $state<number | null>(null);
  let renaming = $state<{ pid: number; index: number } | null>(null);
  let nameDraft = $state("");
  let lastExport = $state("");
  let time = $state(0);
  let playing = $state(false);
  let player: Player | undefined = $state();
  let list: HTMLDivElement | undefined = $state();
  let pendingEdits = $state<Record<number, string>>({});
  let saveError = $state("");
  let saving = $state(0);
  let saveQueue: Promise<void> = Promise.resolve();
  let saveRevision = 0;
  // A newer edit may repeat an older value; only its own save can clear the draft.
  const latestSave: Record<number, number> = {};
  let renameSaving = false;
  const hasPendingEdits = $derived(Object.keys(pendingEdits).length > 0);

  onMount(async () => {
    try {
      project = await api.getProject(app.projectId);
    } catch (e) {
      loadError = String(e);
    }
  });

  const paragraphs = $derived(project?.transcript.paragraphs ?? []);
  const speakers = $derived(project?.transcript.speakers ?? []);
  const activeId = $derived.by(() => {
    const ms = time * 1000;
    let id = -1;
    for (const p of paragraphs) {
      if (p.start_ms <= ms + 50) id = p.id;
      else break;
    }
    return id;
  });

  // ----- search -----
  const q = $derived(query.trim().toLowerCase());
  const matches = $derived.by(() => {
    if (!q) return [] as number[];
    return paragraphs.filter((p) => p.text.toLowerCase().includes(q)).map((p) => p.id);
  });
  const matchCount = $derived.by(() => {
    if (!q) return 0;
    let n = 0;
    for (const p of paragraphs) n += p.text.toLowerCase().split(q).length - 1;
    return n;
  });
  function escapeHtml(s: string) {
    return s.replace(/[&<>"]/g, (c) => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;" })[c]!);
  }
  function highlighted(text: string): string {
    if (!q) return escapeHtml(text);
    const lower = text.toLowerCase();
    let out = "";
    let i = 0;
    for (;;) {
      const j = lower.indexOf(q, i);
      if (j < 0) break;
      out += escapeHtml(text.slice(i, j)) + "<mark>" + escapeHtml(text.slice(j, j + q.length)) + "</mark>";
      i = j + q.length;
    }
    return out + escapeHtml(text.slice(i));
  }
  async function gotoMatch(delta: number) {
    if (!matches.length) return;
    matchIndex = (matchIndex + delta + matches.length) % matches.length;
    await tick();
    list?.querySelector(`[data-pid="${matches[matchIndex]}"]`)?.scrollIntoView({ behavior: "smooth", block: "center" });
  }
  $effect(() => {
    void q;
    matchIndex = 0;
    if (matches.length) tick().then(() => list?.querySelector(`[data-pid="${matches[0]}"]`)?.scrollIntoView({ block: "center" }));
  });

  // ----- follow playback -----
  $effect(() => {
    const id = activeId;
    if (!playing || id < 0 || !list) return;
    const editing = document.activeElement?.closest?.(".ptext");
    if (editing) return;
    const el = list.querySelector(`[data-pid="${id}"]`) as HTMLElement | null;
    if (!el) return;
    const r = el.getBoundingClientRect();
    const lr = list.getBoundingClientRect();
    if (r.top < lr.top || r.bottom > lr.bottom) el.scrollIntoView({ behavior: "smooth", block: "center" });
  });

  // ----- editing -----
  function queueSave(id: number, text: string) {
    const revision = ++saveRevision;
    latestSave[id] = revision;
    saving += 1;
    saveQueue = saveQueue.then(async () => {
      try {
        await api.updateParagraph(project!.id, id, text);
        if (latestSave[id] === revision) {
          delete pendingEdits[id];
          delete latestSave[id];
        }
        if (!Object.keys(pendingEdits).length) saveError = "";
      } catch (e) {
        saveError = String(e);
        toast("Не удалось сохранить правки. Текст остаётся в редакторе — повторите сохранение.", "error");
      } finally {
        saving -= 1;
      }
    });
    return saveQueue;
  }

  async function commitText(p: Paragraph, el: HTMLElement) {
    const text = el.innerText.replace(/\s+\n/g, "\n").trim();
    if (text === p.text) return;
    p.text = text;
    p.edited = true;
    pendingEdits[p.id] = text;
    await queueSave(p.id, text);
  }

  async function retrySave() {
    await Promise.all(Object.entries(pendingEdits).map(([id, text]) => queueSave(Number(id), text)));
  }

  async function flushEdits() {
    await saveQueue;
    if (!Object.keys(pendingEdits).length) return true;
    toast("Сначала сохраните правки: нажмите «Повторить сохранение».", "error");
    return false;
  }

  export async function leave() {
    if (await flushEdits()) app.screen = "main";
  }

  function startRename(p: Paragraph) {
    if (p.speaker == null) return;
    renaming = { pid: p.id, index: p.speaker };
    const current = speakers[p.speaker];
    nameDraft = current === defaultName(p.speaker) ? "" : current;
    tick().then(() => (document.querySelector(".rename input") as HTMLInputElement | null)?.focus());
  }
  async function commitRename() {
    if (!renaming || !project || renameSaving) return;
    renameSaving = true;
    const name = nameDraft.trim() || defaultName(renaming.index);
    try {
      if (!await flushEdits()) return;
      project.transcript = await api.renameSpeaker(project.id, renaming.index, name);
      renaming = null;
    } catch (e) {
      toast(String(e), "error");
    } finally {
      renameSaving = false;
    }
  }
  function defaultName(i: number) {
    return `Спикер ${i + 1}`;
  }

  // ----- actions -----
  async function copyAll() {
    if (!await flushEdits()) return;
    try {
      await writeText(await api.projectText(project!.id));
      toast("Текст скопирован в буфер обмена");
    } catch (e) {
      toast(String(e), "error");
    }
  }
  const formats: { f: ExportFormat; label: string; name: string }[] = [
    { f: "docx", label: "Word", name: "Документ Word" },
    { f: "pdf", label: "PDF", name: "PDF" },
    { f: "txt", label: "TXT", name: "Текст" },
    { f: "srt", label: "SRT", name: "Субтитры SRT" },
  ];
  async function doExport(f: ExportFormat, name: string) {
    exportOpen = false;
    if (!await flushEdits()) return;
    try {
      const defaultPath = await api.defaultExportPath(project!.id, f);
      const path = await save({ defaultPath, filters: [{ name, extensions: [f] }] });
      if (!path) return;
      lastExport = await api.exportProject(project!.id, f, path);
      toast(`Сохранено: ${lastExport.split(/[\\/]/).pop()}`);
    } catch (e) {
      toast(String(e), "error");
    }
  }
  async function copyParagraph(p: Paragraph) {
    menuFor = null;
    await writeText(p.text);
    toast("Абзац скопирован");
  }
  function playFrom(p: Paragraph) {
    menuFor = null;
    player?.seek(p.start_ms / 1000, true);
  }

  function onKey(e: KeyboardEvent) {
    const t = e.target as HTMLElement;
    if (app.settingsOpen || app.aboutOpen) return;
    if (e.key === "Escape") {
      exportOpen = false;
      menuFor = null;
      renaming = null;
      return;
    }
    if ((e.metaKey || e.ctrlKey) && e.key.toLowerCase() === "f") {
      e.preventDefault();
      (document.querySelector(".search input") as HTMLInputElement | null)?.focus();
      return;
    }
    if (t.isContentEditable || t.closest("button, input, textarea, select, a, [role='button']")) return;
    if (e.code === "Space") {
      e.preventDefault();
      player?.toggle();
    }
  }
  function closeMenus(e: MouseEvent) {
    const t = e.target as HTMLElement;
    if (!t.closest(".export")) exportOpen = false;
    if (!t.closest(".pmenu")) menuFor = null;
  }
</script>

<svelte:window onkeydown={onKey} onclick={closeMenus} />

{#if loadError}
  <div class="card errbox">
    <p>{loadError}</p>
    <button class="btn" onclick={() => (app.screen = "main")}>На главный экран</button>
  </div>
{:else if project}
  <div class="wrap">
    <div class="toolbar">
      <button class="btn ghost icon" title="Назад" onclick={leave}><Icon name="arrowLeft" /></button>
      <div class="search">
        <Icon name="search" size={16} />
        <input class="input" placeholder="Поиск по тексту" bind:value={query} onkeydown={(e) => e.key === "Enter" && gotoMatch(e.shiftKey ? -1 : 1)} />
        {#if q}
          <span class="count muted">{matchCount ? `${matches.length ? matchIndex + 1 : 0} из ${matches.length}` : "нет совпадений"}</span>
          <button class="btn ghost icon sm" onclick={() => gotoMatch(-1)} title="Предыдущее"><Icon name="chevronUp" size={15} /></button>
          <button class="btn ghost icon sm" onclick={() => gotoMatch(1)} title="Следующее"><Icon name="chevronDown" size={15} /></button>
          <button class="btn ghost icon sm" onclick={() => (query = "")} title="Очистить"><Icon name="x" size={15} /></button>
        {/if}
      </div>
      <div class="grow"></div>
      {#if lastExport}
        <button class="btn ghost" onclick={() => revealItemInDir(lastExport)} title={lastExport}><Icon name="reveal" size={16} /> Показать файл</button>
      {/if}
      <button class="btn" onclick={copyAll}><Icon name="copy" size={16} /> Копировать всё</button>
      <div class="export">
        <button class="btn primary" aria-expanded={exportOpen} onclick={() => (exportOpen = !exportOpen)}>Экспорт <Icon name="chevronDown" size={16} /></button>
        {#if exportOpen}
          <div class="menu card">
            {#each formats as x (x.f)}
              <button onclick={() => doExport(x.f, x.name)}><span class="ficon" aria-hidden="true">{x.f.toUpperCase()}</span>{x.label}</button>
            {/each}
          </div>
        {/if}
      </div>
    </div>

    {#if hasPendingEdits}
      <div class="save-state" role="status">
        {#if saving}
          <span class="muted">Сохраняем правки…</span>
        {:else if saveError}
          <span>Правки ещё не сохранены. Повторите сохранение перед экспортом или выходом.</span>
          <button class="btn" onclick={retrySave}>Повторить сохранение</button>
        {/if}
      </div>
    {/if}

    <div class="doc-head">
      <div class="dtitle">{project.name}</div>
      <div class="muted small">
        {date(project.created_at)} · {duration(project.transcript.duration_ms)} · {app.info?.languages.find((l) => l.code === project?.transcript.language)?.name ?? project.transcript.language}
        · {project.quality === "accurate" ? "Точно" : "Быстро"}{speakers.length ? ` · спикеров: ${speakers.length}` : ""}
      </div>
    </div>

    <div class="list card" bind:this={list}>
      {#if paragraphs.length === 0}
        <p class="muted empty">Речь не распознана. Попробуйте режим «Точно» или укажите язык вручную.</p>
      {/if}
      {#each paragraphs as p (p.id)}
        <article class="para" class:active={p.id === activeId} class:dim={q && !matches.includes(p.id)} data-pid={p.id}>
          <button class="tc" onclick={() => playFrom(p)} title="Воспроизвести с этого места">{chip(p.start_ms)}</button>
          <div class="body">
            {#if p.speaker != null}
              <div class="phead">
                {#if renaming && renaming.pid === p.id}
                  <span class="spk">{defaultName(p.speaker)}</span>
                  <Icon name="arrowRight" size={14} />
                  <span class="rename">
                    <input
                      bind:value={nameDraft}
                      placeholder="Имя"
                      onkeydown={(e) => {
                        if (e.key === "Enter") commitRename();
                        if (e.key === "Escape") renaming = null;
                      }}
                      onblur={commitRename}
                    />
                    <button title="Отмена" onmousedown={(e) => (e.preventDefault(), (renaming = null))}><Icon name="x" size={13} /></button>
                  </span>
                  <span class="hint faint">Имя изменится для всех упоминаний<br />этого спикера</span>
                {:else}
                  <button class="spkbtn" onclick={() => startRename(p)} title="Переименовать спикера">
                    <span class="spk">{defaultName(p.speaker)}</span>
                    {#if speakers[p.speaker] !== defaultName(p.speaker)}
                      <Icon name="arrowRight" size={14} />
                      <span class="named">{speakers[p.speaker]}</span>
                    {/if}
                    <span class="pen"><Icon name="edit" size={13} /></span>
                  </button>
                {/if}
              </div>
            {/if}
            {#if q}
              <div class="ptext ro">{@html highlighted(p.text)}</div>
            {:else}
              <div
                class="ptext"
                contenteditable="plaintext-only"
                spellcheck="true"
                role="textbox"
                tabindex="0"
                onblur={(e) => commitText(p, e.currentTarget as HTMLElement)}
              >{p.text}</div>
            {/if}
          </div>
          <div class="pmenu">
            <button class="btn ghost icon sm" onclick={() => (menuFor = menuFor === p.id ? null : p.id)} title="Ещё"><Icon name="more" size={18} stroke={3} /></button>
            {#if menuFor === p.id}
              <div class="menu card right">
                <button onclick={() => playFrom(p)}><Icon name="play" size={13} /> Воспроизвести отсюда</button>
                <button onclick={() => copyParagraph(p)}><Icon name="copy" size={14} /> Копировать абзац</button>
                {#if p.speaker != null}
                  <button onclick={() => ((menuFor = null), startRename(p))}><Icon name="edit" size={14} /> Переименовать спикера</button>
                {/if}
              </div>
            {/if}
          </div>
        </article>
      {/each}
    </div>

    <Player bind:this={player} src={mediaUrl(project.media_path)} totalMs={project.transcript.duration_ms} bind:time bind:playing />
  </div>
{/if}

<style>
  .wrap {
    height: 100%;
    display: flex;
    flex-direction: column;
    gap: 12px;
  }
  .toolbar {
    display: flex;
    align-items: center;
    gap: 10px;
    flex-wrap: wrap;
  }
  .save-state {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    font-size: 13px;
    color: var(--danger);
  }
  .search {
    position: relative;
    display: flex;
    align-items: center;
    gap: 4px;
    color: var(--muted);
  }
  .search :global(svg:first-child) {
    position: absolute;
    left: 11px;
  }
  .search .input {
    width: 300px;
    padding-left: 34px;
  }
  .count {
    font-size: 12.5px;
    margin: 0 4px 0 8px;
    white-space: nowrap;
  }
  .btn.sm {
    width: 28px;
    height: 28px;
  }
  .grow {
    flex: 1;
  }
  .export {
    position: relative;
  }
  .menu {
    position: absolute;
    top: calc(100% + 6px);
    right: 0;
    min-width: 180px;
    padding: 6px;
    z-index: 20;
    box-shadow: var(--shadow-lg);
    display: flex;
    flex-direction: column;
  }
  .menu button {
    display: flex;
    align-items: center;
    gap: 10px;
    border: 0;
    background: none;
    padding: 8px 10px;
    border-radius: 8px;
    cursor: pointer;
    text-align: left;
    white-space: nowrap;
  }
  .menu button:hover {
    background: var(--primary-soft);
  }
  .ficon {
    font-size: 9px;
    font-weight: 700;
    color: var(--primary);
    border: 1.5px solid currentColor;
    border-radius: 4px;
    padding: 2px 3px;
    min-width: 34px;
    text-align: center;
  }
  .doc-head {
    padding: 0 4px;
  }
  .dtitle {
    font-weight: 600;
    font-size: 15px;
  }
  .small {
    font-size: 12.5px;
  }
  .list {
    flex: 1;
    min-height: 0;
    overflow: auto;
    padding: 6px 10px;
  }
  .empty {
    padding: 30px;
    text-align: center;
  }
  .para {
    display: grid;
    grid-template-columns: 92px 1fr 36px;
    gap: 10px;
    padding: 14px 8px;
    border-bottom: 1px solid var(--border);
    border-radius: 10px;
    transition: background 0.2s, opacity 0.2s;
  }
  .para:last-child {
    border-bottom: 0;
  }
  .para.active {
    background: var(--active-para);
  }
  .para.dim {
    opacity: 0.45;
  }
  .tc {
    align-self: start;
    border: 0;
    background: var(--primary-soft);
    color: var(--primary-text);
    font-weight: 600;
    font-size: 12.5px;
    font-variant-numeric: tabular-nums;
    padding: 5px 9px;
    border-radius: 8px;
    cursor: pointer;
    justify-self: start;
  }
  .tc:hover {
    filter: brightness(0.96);
  }
  .body {
    min-width: 0;
  }
  .phead {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-bottom: 4px;
    min-height: 28px;
  }
  .spk {
    font-weight: 600;
  }
  .spkbtn {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    border: 0;
    background: none;
    padding: 2px 6px;
    margin-left: -6px;
    border-radius: 6px;
    cursor: pointer;
  }
  .spkbtn:hover {
    background: var(--card-2);
  }
  .spkbtn .pen {
    opacity: 0;
    color: var(--muted);
    display: flex;
  }
  .spkbtn:hover .pen {
    opacity: 1;
  }
  .named {
    color: var(--primary-text);
    font-weight: 600;
  }
  .rename {
    display: inline-flex;
    align-items: center;
    border: 1.5px solid var(--primary);
    border-radius: 8px;
    background: var(--primary-soft);
    padding: 0 4px 0 8px;
    height: 28px;
  }
  .rename input {
    border: 0;
    outline: 0;
    background: transparent;
    width: 140px;
    color: var(--primary-text);
    font-weight: 600;
    user-select: text;
    -webkit-user-select: text;
  }
  .rename button {
    border: 0;
    background: none;
    cursor: pointer;
    color: var(--muted);
    display: flex;
  }
  .hint {
    font-size: 11.5px;
    line-height: 1.25;
    margin-left: 6px;
  }
  .ptext {
    font-size: 14.5px;
    line-height: 1.6;
    color: var(--text-2);
    outline: none;
    border-radius: 6px;
    padding: 2px 4px;
    margin-left: -4px;
    white-space: pre-wrap;
    user-select: text;
    -webkit-user-select: text;
    cursor: text;
  }
  .ptext:focus {
    background: var(--card-2);
    box-shadow: 0 0 0 2px color-mix(in srgb, var(--primary) 25%, transparent);
  }
  .ptext :global(mark) {
    background: var(--mark);
    color: inherit;
    border-radius: 3px;
    padding: 0 1px;
  }
  .pmenu {
    position: relative;
    color: var(--muted);
  }
  .menu.right {
    right: 0;
    min-width: 230px;
  }
  .errbox {
    max-width: 520px;
    margin: 60px auto;
    padding: 24px;
  }
</style>
