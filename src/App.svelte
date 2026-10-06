<script lang="ts">
  import { onMount } from "svelte";
  import { app, init } from "./lib/state.svelte";
  import { isMac } from "./lib/format";
  import Icon from "./components/Icon.svelte";
  import Main from "./screens/Main.svelte";
  import Processing from "./screens/Processing.svelte";
  import Editor from "./screens/Editor.svelte";
  import FirstRun from "./screens/FirstRun.svelte";
  import Settings from "./screens/Settings.svelte";
  import About from "./screens/About.svelte";
  import logo from "./assets/logo.png";

  let error = $state("");
  let editor: Editor | undefined = $state();
  onMount(() => {
    init().catch((e) => (error = String(e)));
    // block the browser context menu / file-open on stray drops
    const prevent = (e: Event) => e.preventDefault();
    window.addEventListener("dragover", prevent);
    window.addEventListener("drop", prevent);
    return () => {
      window.removeEventListener("dragover", prevent);
      window.removeEventListener("drop", prevent);
    };
  });

  function goHome() {
    if (app.screen === "editor") void editor?.leave();
  }
</script>

<div class="shell" class:mac={isMac}>
  <header data-tauri-drag-region>
    <button class="brand" onclick={goHome} title="На главный экран">
      <img src={logo} alt="" width="22" height="22" />
      <span>Кнорозов PRO</span>
    </button>
    <div class="spacer" data-tauri-drag-region></div>
    <button class="btn ghost icon" title="Настройки" onclick={() => ((app.settingsSection = ""), (app.settingsOpen = true))}>
      <Icon name="gear" size={18} />
    </button>
  </header>

  <main>
    {#if error}
      <div class="fatal card">
        <h3>Не удалось запустить приложение</h3>
        <p class="muted">{error}</p>
      </div>
    {:else if app.screen === "firstrun"}
      <FirstRun />
    {:else if app.screen === "main"}
      <Main />
    {:else if app.screen === "processing"}
      <Processing />
    {:else if app.screen === "editor"}
      <Editor bind:this={editor} />
    {/if}
  </main>

  {#if app.settingsOpen}
    <Settings />
  {/if}
  {#if app.aboutOpen}
    <About />
  {/if}
  {#if app.toast}
    <div class="toast" class:error={app.toastKind === "error"} role="status">{app.toast}</div>
  {/if}
</div>

<style>
  .shell {
    height: 100%;
    display: flex;
    flex-direction: column;
  }
  header {
    height: 46px;
    flex: none;
    display: flex;
    align-items: center;
    padding: 0 12px 0 14px;
    gap: 8px;
  }
  .mac header {
    padding-left: 84px;
  }
  .brand {
    display: flex;
    align-items: center;
    gap: 8px;
    border: 0;
    background: none;
    cursor: pointer;
    font-weight: 600;
    font-size: 13.5px;
    padding: 4px 6px;
    border-radius: 8px;
  }
  .brand img {
    border-radius: 6px;
  }
  .spacer {
    flex: 1;
    height: 100%;
  }
  main {
    flex: 1;
    min-height: 0;
    padding: 0 18px 18px;
  }
  .fatal {
    max-width: 520px;
    margin: 80px auto;
    padding: 24px;
  }
  .toast {
    position: fixed;
    left: 50%;
    bottom: 26px;
    transform: translateX(-50%);
    background: #111827;
    color: #fff;
    padding: 10px 16px;
    border-radius: 10px;
    box-shadow: var(--shadow-lg);
    font-size: 13.5px;
    max-width: 640px;
    z-index: 100;
    user-select: text;
    -webkit-user-select: text;
  }
  .toast.error {
    background: #b91c1c;
  }
</style>
