<script lang="ts">
  // Document-with-waveform icon used on the processing screen and in lists.
  let { size = 72 }: { size?: number } = $props();
</script>

<svg width={size} height={size * 1.2} viewBox="0 0 60 72" aria-hidden="true">
  <defs>
    <linearGradient id="fb" x1="0" y1="0" x2="0" y2="1">
      <stop offset="0" stop-color="#ffffff" />
      <stop offset="1" stop-color="#eef3fb" />
    </linearGradient>
  </defs>
  <path d="M8 2h30l16 16v48a4 4 0 0 1-4 4H8a4 4 0 0 1-4-4V6a4 4 0 0 1 4-4Z" fill="url(#fb)" stroke="#d6dfec" stroke-width="1.5" />
  <path d="M38 2v12a4 4 0 0 0 4 4h12" fill="#e3ebf7" stroke="#d6dfec" stroke-width="1.5" />
  <g stroke="#3b82f6" stroke-width="3" stroke-linecap="round">
    <path d="M17 40v4" /><path d="M23 34v16" /><path d="M29 30v24" /><path d="M35 36v12" /><path d="M41 39v6" />
  </g>
</svg>
