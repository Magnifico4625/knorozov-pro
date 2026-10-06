<script lang="ts">
  // Minimal stroke icon set (24×24, drawn for this app).
  let { name, size = 18, stroke = 1.8 }: { name: string; size?: number; stroke?: number } = $props();

  const paths: Record<string, string> = {
    gear: "M12 15a3 3 0 1 0 0-6 3 3 0 0 0 0 6Z M19.4 15a1.7 1.7 0 0 0 .3 1.8l.1.1a2 2 0 1 1-2.8 2.8l-.1-.1a1.7 1.7 0 0 0-1.8-.3 1.7 1.7 0 0 0-1 1.5V21a2 2 0 1 1-4 0v-.1a1.7 1.7 0 0 0-1.1-1.5 1.7 1.7 0 0 0-1.8.3l-.1.1a2 2 0 1 1-2.8-2.8l.1-.1a1.7 1.7 0 0 0 .3-1.8 1.7 1.7 0 0 0-1.5-1H3a2 2 0 1 1 0-4h.1a1.7 1.7 0 0 0 1.5-1.1 1.7 1.7 0 0 0-.3-1.8l-.1-.1a2 2 0 1 1 2.8-2.8l.1.1a1.7 1.7 0 0 0 1.8.3H9a1.7 1.7 0 0 0 1-1.5V3a2 2 0 1 1 4 0v.1a1.7 1.7 0 0 0 1 1.5 1.7 1.7 0 0 0 1.8-.3l.1-.1a2 2 0 1 1 2.8 2.8l-.1.1a1.7 1.7 0 0 0-.3 1.8V9a1.7 1.7 0 0 0 1.5 1H21a2 2 0 1 1 0 4h-.1a1.7 1.7 0 0 0-1.5 1Z",
    search: "M11 19a8 8 0 1 0 0-16 8 8 0 0 0 0 16Z M21 21l-4.3-4.3",
    copy: "M9 9h11v11H9z M5 15H4V4h11v1",
    chevronDown: "M6 9l6 6 6-6",
    chevronUp: "M18 15l-6-6-6 6",
    chevronRight: "M9 6l6 6-6 6",
    arrowRight: "M5 12h14 M13 6l6 6-6 6",
    arrowLeft: "M19 12H5 M11 18l-6-6 6-6",
    x: "M18 6 6 18 M6 6l12 12",
    check: "M20 6 9 17l-5-5",
    shield: "M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10Z M9 12l2 2 4-4",
    folder: "M3 7a2 2 0 0 1 2-2h4l2 2h8a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2Z",
    download: "M12 3v12 M7 10l5 5 5-5 M5 21h14",
    theme: "M12 3a9 9 0 1 0 9 9 7 7 0 0 1-9-9Z",
    more: "M5 12h.01 M12 12h.01 M19 12h.01",
    volume: "M11 5 6 9H2v6h4l5 4V5Z M15.5 8.5a5 5 0 0 1 0 7 M19 5a10 10 0 0 1 0 14",
    mute: "M11 5 6 9H2v6h4l5 4V5Z M22 9l-6 6 M16 9l6 6",
    fullscreen: "M8 3H5a2 2 0 0 0-2 2v3 M21 8V5a2 2 0 0 0-2-2h-3 M3 16v3a2 2 0 0 0 2 2h3 M16 21h3a2 2 0 0 0 2-2v-3",
    trash: "M3 6h18 M8 6V4h8v2 M6 6l1 14h10l1-14",
    info: "M12 22a10 10 0 1 0 0-20 10 10 0 0 0 0 20Z M12 16v-4 M12 8h.01",
    file: "M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8Z M14 2v6h6",
    audio: "M9 18V5l12-2v13 M6 21a3 3 0 1 0 0-6 3 3 0 0 0 0 6Z M18 19a3 3 0 1 0 0-6 3 3 0 0 0 0 6Z",
    video: "M15 10l5-3v10l-5-3Z M3 6h12v12H3z",
    mic: "M12 15a3 3 0 0 0 3-3V5a3 3 0 0 0-6 0v7a3 3 0 0 0 3 3Z M19 11a7 7 0 0 1-14 0 M12 18v4",
    play: "M7 4l13 8-13 8Z",
    pause: "M7 4h3v16H7z M14 4h3v16h-3z",
    edit: "M12 20h9 M16.5 3.5a2.1 2.1 0 0 1 3 3L7 19l-4 1 1-4Z",
    reveal: "M14 3h7v7 M10 14 21 3 M19 14v5a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V7a2 2 0 0 1 2-2h5",
  };
  const filled = new Set(["play", "pause"]);
  const d = $derived(paths[name] ?? paths.info);
</script>

<svg
  width={size}
  height={size}
  viewBox="0 0 24 24"
  fill={filled.has(name) ? "currentColor" : "none"}
  stroke={filled.has(name) ? "none" : "currentColor"}
  stroke-width={stroke}
  stroke-linecap="round"
  stroke-linejoin="round"
  aria-hidden="true"
>
  {#each d.split(" M") as seg, i (i)}
    <path d={i === 0 ? seg : "M" + seg} />
  {/each}
</svg>
