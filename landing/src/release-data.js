export const repository = "Magnifico4625/knorozov-pro";
export const releasesUrl = `https://github.com/${repository}/releases`;

export function installerManifest(releases) {
  const empty = { version: null, windows: null, mac: null, pkg: null, releaseUrl: releasesUrl };
  if (!Array.isArray(releases)) return empty;
  for (const release of releases) {
    if (!release || release.draft || release.prerelease || !/^v\d+\.\d+\.\d+$/.test(release.tag_name ?? "")) continue;
    const version = release.tag_name.slice(1);
    const asset = name => {
      const match = Array.isArray(release.assets) ? release.assets.find(item => item?.name === name && item.state === "uploaded" && item.size > 0) : null;
      if (!match) return null;
      let url;
      try { url = new URL(match.browser_download_url); } catch { return null; }
      if (url.protocol !== "https:" || url.hostname !== "github.com" || url.username || url.password || !url.pathname.startsWith(`/${repository}/releases/download/`)) return null;
      return { url: url.href, name, size: match.size };
    };
    const windows = asset(`Knorozov-PRO_${version}_Windows-x64-setup.exe`);
    const mac = asset(`Knorozov-PRO_${version}_macOS-arm64.dmg`);
    if (!windows || !mac) continue;
    return { version, windows, mac,
      pkg: asset(`Knorozov-PRO_${version}_macOS-arm64.pkg`),
      releaseUrl: `${releasesUrl}/tag/${release.tag_name}` };
  }
  return empty;
}
