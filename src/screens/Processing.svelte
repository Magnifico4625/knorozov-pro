<script lang="ts">
  import { tick } from "svelte";
  import FileBadge from "../components/FileBadge.svelte";
  import { api } from "../lib/api";
  import { app } from "../lib/state.svelte";
  import { duration, eta } from "../lib/format";

  let box: HTMLDivElement | undefined = $state();
  let cancelling = $state(false);

  const pct = $derived(Math.max(0, Math.min(100, Math.round(app.progress?.percent ?? 0))));
  const stageText = $derived.by(() => {
    switch (app.progress?.stage) {
      case "decoding":
        return "Подготавливаем звук…";
      case "loading":
        return "Загружаем модель…";
      case "diarizing":
        return "Определяем спикеров…";
      case "saving":
        return "Сохраняем…";
      default:
        return "Идёт расшифровка…";
    }
  });
  const etaText = $derived.by(() => {
    const s = app.progress?.stage;
    if (s === "decoding" || s === "loading") return "Оцениваем время…";
    if (s === "diarizing" || s === "saving") return "Почти готово";
    return eta(app.progress?.eta_sec ?? null);
  });

  $effect(() => {
    void app.segments.length;
    tick().then(() => box?.scrollTo({ top: box.scrollHeight, behavior: "smooth" }));
  });

  async function cancel() {
    cancelling = true;
    await api.cancelTranscription();
  }
</script>

<div class="wrap">
  <section class="card top">
    <FileBadge size={64} />
    <div class="info">
      <div class="name" title={app.file?.name}>{app.file?.name ?? "Файл"}</div>
      <div class="muted meta">
        {app.file?.format ?? ""}{#if app.file?.duration_sec} · {duration(app.file.duration_sec * 1000)}{/if}
      </div>
      <div class="progress"><div style="width: {pct}%"></div></div>
      <div class="stats">
        <span class="muted">{app.settings?.quality === "accurate" ? "Режим «Точно»" : "Режим «Быстро»"}{app.settings?.diarize ? " · спикеры" : ""}</span>
        <span class="right"><b>{pct}%</b><br /><span class="muted">{etaText}</span></span>
      </div>
    </div>
  </section>

  <section class="card live">
    <h3>{stageText}</h3>
    <div class="text" bind:this={box}>
      {#if app.segments.length === 0}
        <p class="faint">Текст появится здесь по мере распознавания.</p>
      {:else}
        {#each app.segments as s, i (i)}
          <span class:latest={i === app.segments.length - 1}>{s.text} </span>
        {/each}
        <span class="caret"></span>
      {/if}
    </div>
    <div class="actions">
      <button class="btn" onclick={cancel} disabled={cancelling}>{cancelling ? "Отменяем…" : "Отмена"}</button>
    </div>
  </section>
</div>

<style>
  .wrap {
    height: 100%;
    display: flex;
    flex-direction: column;
    gap: 16px;
    max-width: 980px;
    margin: 0 auto;
  }
  .top {
    display: flex;
    gap: 22px;
    padding: 22px 26px;
    align-items: center;
  }
  .info {
    flex: 1;
    min-width: 0;
  }
  .name {
    font-size: 19px;
    font-weight: 600;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .meta {
    margin: 2px 0 12px;
    font-size: 13px;
  }
  .stats {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-top: 8px;
    font-size: 13px;
  }
  .stats .right {
    text-align: right;
  }
  .live {
    flex: 1;
    min-height: 0;
    display: flex;
    flex-direction: column;
    padding: 18px 24px;
  }
  h3 {
    margin: 0 0 10px;
    font-size: 15px;
  }
  .text {
    flex: 1;
    overflow: auto;
    font-size: 15px;
    line-height: 1.7;
    color: var(--muted);
    user-select: text;
    -webkit-user-select: text;
    padding-right: 6px;
  }
  .text .latest {
    color: var(--text);
  }
  .caret {
    display: inline-block;
    width: 2px;
    height: 1.1em;
    background: var(--text);
    vertical-align: text-bottom;
    animation: blink 1s steps(2) infinite;
  }
  @keyframes blink {
    50% {
      opacity: 0;
    }
  }
  .actions {
    display: flex;
    justify-content: flex-end;
    padding-top: 12px;
    border-top: 1px solid var(--border);
    margin-top: 12px;
  }
  .actions .btn {
    min-width: 120px;
  }
</style>
