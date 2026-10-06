import "./style.css";
import { installerManifest, repository, releasesUrl } from "./release-data.js";

const status = document.querySelector("#release-status");
let available = false;

function applyManifest(manifest) {
  if (!manifest.version || !manifest.windows || !manifest.mac) return;
  available = true;
  status.textContent = `Версия ${manifest.version} · установщики готовы к скачиванию`;
  for (const platform of ["windows", "mac"]) {
    const asset = manifest[platform];
    for (const link of document.querySelectorAll(`[data-download="${platform}"]`)) {
      link.href = asset.url;
      link.setAttribute("aria-label", `Скачать ${asset.name}`);
      if (link.hasAttribute("data-release-card")) link.querySelector("span").textContent = "Скачать из GitHub Releases";
    }
    document.querySelector(`[data-asset-detail="${platform}"]`).textContent = `Версия ${manifest.version} · ${(asset.size / 1048576).toFixed(1)} МБ`;
  }
  const pkg = document.querySelector("#pkg-download");
  pkg.hidden = !manifest.pkg;
  if (manifest.pkg) {
    pkg.href = manifest.pkg.url;
  }
}

async function loadRelease() {
  try {
    const cached = await fetch(`${import.meta.env.BASE_URL}releases.json`);
    if (cached.ok) {
      // Validate the static manifest through the same allowlist as live releases.
      const item = await cached.json();
      if (item.version) applyManifest(installerManifest([{ tag_name: `v${item.version}`, assets: [item.windows, item.mac, item.pkg].filter(Boolean).map(asset => ({name: asset.name, size: asset.size, state: "uploaded", browser_download_url: asset.url})) }]));
    }
  } catch { /* The public release list can still provide the download links. */ }
  try {
    const response = await fetch(`https://api.github.com/repos/${repository}/releases?per_page=30`, {
      headers: { Accept: "application/vnd.github+json" }, signal: AbortSignal.timeout(8000),
    });
    if (!response.ok) throw new Error("Release list unavailable");
    applyManifest(installerManifest(await response.json()));
    if (!available) status.textContent = "Первый выпуск ещё готовится. Когда установщики появятся в Releases, здесь будут ссылки на скачивание.";
  } catch {
    if (!available) status.textContent = "Проверить выпуск сейчас не удалось. Все доступные версии можно посмотреть в GitHub Releases.";
  }
  if (!available) {
    for (const link of document.querySelectorAll("[data-release-card]")) link.href = releasesUrl;
  }
}

loadRelease();
