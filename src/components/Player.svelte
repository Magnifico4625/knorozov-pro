<script lang="ts">
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import Icon from "./Icon.svelte";
  import { chip, duration } from "../lib/format";

  let {
    src,
    totalMs,
    time = $bindable(0),
    playing = $bindable(false),
  }: { src: string; totalMs: number; time?: number; playing?: boolean } = $props();

  let audio: HTMLAudioElement | undefined = $state();
  let dur = $state(0);
  let volume = $state(1);
  let muted = $state(false);
  let rate = $state(1);
  let failed = $state(false);
  const rates = [0.75, 1, 1.25, 1.5, 2];

  const total = $derived(dur > 0 && isFinite(dur) ? dur : totalMs / 1000);

  export function seek(sec: number, play = true) {
    if (!audio) return;
    audio.currentTime = Math.max(0, sec);
    time = audio.currentTime;
    if (play) audio.play().catch(() => {});
  }
  export function toggle() {
    if (!audio) return;
    if (audio.paused) audio.play().catch(() => (failed = true));
    else audio.pause();
  }

  function onScrub(e: Event) {
    const v = Number((e.target as HTMLInputElement).value);
    seek(v, !audio?.paused);
  }
  function cycleRate() {
    rate = rates[(rates.indexOf(rate) + 1) % rates.length];
    if (audio) audio.playbackRate = rate;
  }
  async function fullscreen() {
    const w = getCurrentWindow();
    await w.setFullscreen(!(await w.isFullscreen()));
  }
  $effect(() => {
    if (audio) {
      audio.volume = volume;
      audio.muted = muted;
    }
  });
</script>

<div class="player card">
  <audio
    bind:this={audio}
    {src}
    preload="metadata"
    ontimeupdate={() => (time = audio!.currentTime)}
    onloadedmetadata={() => ((dur = audio!.duration), (failed = false))}
    onplay={() => (playing = true)}
    onpause={() => (playing = false)}
    onended={() => (playing = false)}
    onerror={() => (failed = true)}
  ></audio>
  <button class="play" onclick={toggle} disabled={failed} title={playing ? "Пауза" : "Воспроизвести"}>
    <Icon name={playing ? "pause" : "play"} size={16} />
  </button>
  <div class="time">
    <b>{chip(time * 1000)}</b> <span class="muted">/ {duration(total * 1000)}</span>
  </div>
  {#if failed}
    <div class="muted fail">Исходный файл недоступен — воспроизведение невозможно, текст можно редактировать и экспортировать.</div>
  {:else}
    <input
      class="scrub"
      type="range"
      min="0"
      max={total || 1}
      step="0.1"
      value={time}
      oninput={onScrub}
      style="--p: {total ? (time / total) * 100 : 0}%"
      aria-label="Позиция"
    />
  {/if}
  <button class="btn ghost icon" onclick={() => (muted = !muted)} title="Звук"><Icon name={muted ? "mute" : "volume"} /></button>
  <input class="vol" type="range" min="0" max="1" step="0.01" bind:value={volume} style="--p: {volume * 100}%" aria-label="Громкость" />
  <button class="btn rate" onclick={cycleRate} title="Скорость">{String(rate).replace(".", ",")}×</button>
  <button class="btn ghost icon" onclick={fullscreen} title="Во весь экран"><Icon name="fullscreen" /></button>
</div>

<style>
  .player {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 10px 14px;
  }
  .play {
    width: 36px;
    height: 36px;
    border-radius: 10px;
    border: 0;
    background: linear-gradient(180deg, #4a90ff, var(--primary));
    color: #fff;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    flex: none;
  }
  .play:disabled {
    opacity: 0.4;
  }
  .time {
    font-variant-numeric: tabular-nums;
    white-space: nowrap;
    font-size: 13px;
  }
  .fail {
    flex: 1;
    font-size: 12.5px;
  }
  input[type="range"] {
    -webkit-appearance: none;
    appearance: none;
    height: 4px;
    border-radius: 4px;
    background: linear-gradient(90deg, var(--primary) var(--p), var(--border-strong) var(--p));
    outline: none;
    cursor: pointer;
  }
  input[type="range"]::-webkit-slider-thumb {
    -webkit-appearance: none;
    width: 14px;
    height: 14px;
    border-radius: 50%;
    background: var(--primary);
    border: 2px solid #fff;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.3);
  }
  .scrub {
    flex: 1;
  }
  .vol {
    width: 90px;
  }
  .rate {
    height: 30px;
    padding: 0 10px;
    font-variant-numeric: tabular-nums;
  }
</style>
