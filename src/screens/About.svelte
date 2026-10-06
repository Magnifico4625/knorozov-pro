<script lang="ts">
  import { openUrl } from "@tauri-apps/plugin-opener";
  import Icon from "../components/Icon.svelte";
  import { app } from "../lib/state.svelte";
  import { LICENSES } from "../lib/licenses";
  import { modalDialog } from "../lib/dialog";
  import logo from "../assets/logo.png";
</script>

<div class="backdrop" role="presentation" onclick={() => (app.aboutOpen = false)}></div>
<div class="modal card" role="dialog" aria-modal="true" aria-label="О программе" tabindex="-1" use:modalDialog={() => (app.aboutOpen = false)}>
  <button class="btn ghost icon close" onclick={() => (app.aboutOpen = false)} title="Закрыть"><Icon name="x" /></button>
  <div class="top">
    <img src={logo} alt="" width="64" height="64" />
    <div>
      <h2>Кнорозов PRO</h2>
      <div class="muted">Версия {app.info?.version}</div>
    </div>
  </div>
  <p>
    Офлайн-расшифровка аудио и видео в текст. Всё распознавание выполняется на этом компьютере — записи никуда не
    отправляются.
  </p>
  <p class="muted small">
    Названа в честь Юрия Валентиновича Кнорозова (1922–1999) — советского учёного, расшифровавшего письменность майя.
  </p>
  <h3>Сторонние компоненты и лицензии</h3>
  <div class="list">
    {#each LICENSES as l (l.name)}
      <button class="row" onclick={() => openUrl(l.url)} title={l.url}>
        <span class="name">{l.name}{#if l.note}<span class="faint"> — {l.note}</span>{/if}</span>
        <span class="lic">{l.license}</span>
      </button>
    {/each}
  </div>
  <p class="faint small">Полные тексты лицензий — в файле THIRD_PARTY_LICENSES.md рядом с программой.</p>
</div>

<style>
  .backdrop {
    position: fixed;
    inset: 0;
    background: rgba(15, 23, 42, 0.3);
    z-index: 50;
  }
  .modal {
    position: fixed;
    left: 50%;
    top: 50%;
    transform: translate(-50%, -50%);
    width: 560px;
    max-height: 86vh;
    overflow: auto;
    z-index: 51;
    padding: 24px 26px;
    box-shadow: var(--shadow-lg);
    user-select: text;
    -webkit-user-select: text;
  }
  .close {
    position: absolute;
    right: 12px;
    top: 12px;
  }
  .top {
    display: flex;
    gap: 16px;
    align-items: center;
  }
  .top img {
    border-radius: 14px;
  }
  h2 {
    margin: 0;
  }
  h3 {
    font-size: 14px;
    margin: 18px 0 8px;
  }
  .small {
    font-size: 12.5px;
  }
  .list {
    border: 1px solid var(--border);
    border-radius: 10px;
    overflow: hidden;
  }
  .row {
    display: flex;
    width: 100%;
    justify-content: space-between;
    gap: 12px;
    padding: 8px 12px;
    border: 0;
    border-bottom: 1px solid var(--border);
    background: none;
    cursor: pointer;
    text-align: left;
    font-size: 13px;
  }
  .row:last-child {
    border-bottom: 0;
  }
  .row:hover {
    background: var(--card-2);
  }
  .lic {
    color: var(--muted);
    white-space: nowrap;
  }
</style>
