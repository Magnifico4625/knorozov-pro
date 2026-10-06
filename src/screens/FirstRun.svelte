<script lang="ts">
  import { onMount } from "svelte";
  import Icon from "../components/Icon.svelte";
  import { api } from "../lib/api";
  import { app, saveSettings, startDownload } from "../lib/state.svelte";
  import { mb, speed } from "../lib/format";
  import art from "../assets/first-launch.png";

  const totalBytes = $derived(app.models.filter((m) => !m.installed).reduce((s, m) => s + m.size, 0));
  const doneBefore = $derived.by(() => {
    const p = app.download.progress;
    if (!p) return 0;
    // bytes of models in the queue already finished
    return app.models
      .filter((m) => app.download.queue.slice(0, p.index).includes(m.quality))
      .reduce((s, m) => s + m.size, 0);
  });
  const queueTotal = $derived(
    app.models.filter((m) => app.download.queue.includes(m.quality)).reduce((s, m) => s + m.size, 0) || totalBytes,
  );
  const got = $derived(doneBefore + (app.download.progress?.downloaded ?? 0));
  const pct = $derived(queueTotal ? Math.min(100, Math.round((got / queueTotal) * 100)) : 0);

  onMount(() => {
    if (!app.download.running && !app.download.error) startDownload(["fast", "accurate"]);
  });

  async function cancel() {
    await api.cancelDownload();
  }
  function retry() {
    startDownload(["fast", "accurate"]);
  }
  function skip() {
    app.screen = "main";
  }
  async function useMirror(url: string) {
    if (!app.settings) return;
    app.settings.mirror = url;
    await saveSettings();
    retry();
  }
  function openSettings(section: string) {
    app.settingsSection = section;
    app.settingsOpen = true;
  }
</script>

<div class="grid card">
  <section class="main">
    <img class="illustration art" src={art} alt="" draggable="false" />
    <h1>Скачиваем модели распознавания<br /><span>({mb(queueTotal)})</span></h1>

    {#if app.download.error}
      <div class="error">
        <p><b>Не удалось скачать модель.</b><br /><span class="muted">{app.download.error}</span></p>
        <p class="muted small">Если Hugging Face недоступен, попробуйте зеркало — загрузка продолжится с того же места.</p>
        <div class="buttons">
          <button class="btn primary" onclick={retry}>Повторить</button>
          <button class="btn" onclick={() => useMirror("https://hf-mirror.com")}>Через зеркало hf-mirror.com</button>
          <button class="btn ghost" onclick={() => openSettings("models")}>Другое зеркало…</button>
        </div>
      </div>
    {:else}
      <div class="bar">
        <div class="progress"><div style="width: {pct}%"></div></div>
        <span class="pct">{pct}%</span>
      </div>
      <p class="muted small">
        {#if app.download.running}
          Пожалуйста, подождите. Это может занять несколько минут.
          {#if app.download.progress}<br />{mb(got)} из {mb(queueTotal)} · {speed(app.download.progress.speed)}{/if}
        {:else}
          Загрузка остановлена. Её можно продолжить в любой момент.
        {/if}
      </p>
      <div class="buttons">
        {#if app.download.running}
          <button class="btn" onclick={cancel}>Отмена</button>
        {:else}
          <button class="btn primary" onclick={retry}>Продолжить загрузку</button>
          {#if app.models.some((m) => m.installed)}
            <button class="btn" onclick={skip}>Перейти к работе</button>
          {/if}
        {/if}
      </div>
    {/if}

    <span class="badge-offline"><Icon name="shield" size={16} /> Работает офлайн, ваши записи никуда не отправляются</span>
  </section>

  <aside class="side">
    <h3><Icon name="gear" size={18} /> Настройки</h3>
    <button class="item" onclick={() => openSettings("folder")}><Icon name="folder" /> <span>папка для сохранения</span> <Icon name="chevronRight" size={16} /></button>
    <button class="item" onclick={() => openSettings("models")}><Icon name="download" /> <span>загрузка моделей</span> <Icon name="chevronRight" size={16} /></button>
    <button class="item" onclick={() => openSettings("theme")}><Icon name="theme" /> <span>тема светлая или тёмная</span> <Icon name="chevronRight" size={16} /></button>
  </aside>
</div>

<style>
  .grid {
    height: 100%;
    display: grid;
    grid-template-columns: 1fr 300px;
    overflow: hidden;
  }
  .main {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 14px;
    padding: 24px 40px;
    text-align: center;
    overflow: auto;
  }
  .art {
    width: min(160px, 40%);
    height: auto;
  }
  h1 {
    font-size: 22px;
    line-height: 1.3;
    margin: 0;
  }
  h1 span {
    font-weight: 500;
  }
  .bar {
    display: flex;
    align-items: center;
    gap: 14px;
    width: min(520px, 100%);
  }
  .bar .progress {
    flex: 1;
  }
  .pct {
    font-weight: 600;
    width: 44px;
    text-align: right;
    color: var(--text-2);
  }
  .small {
    font-size: 13px;
    margin: 0;
  }
  .buttons {
    display: flex;
    gap: 10px;
    flex-wrap: wrap;
    justify-content: center;
  }
  .error {
    max-width: 560px;
    user-select: text;
    -webkit-user-select: text;
  }
  .side {
    padding: 18px;
    display: flex;
    flex-direction: column;
    gap: 10px;
    border-left: 1px solid var(--border);
  }
  h3 {
    display: flex;
    align-items: center;
    gap: 8px;
    margin: 0 0 6px;
    font-size: 15px;
  }
  .item {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 12px;
    border: 1px solid var(--border);
    border-radius: 10px;
    background: var(--card);
    cursor: pointer;
    text-align: left;
  }
  .item:hover {
    background: var(--card-2);
  }
  .item span {
    flex: 1;
  }
</style>
