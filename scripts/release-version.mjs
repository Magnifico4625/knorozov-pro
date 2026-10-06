import { readFileSync, appendFileSync } from "node:fs";
import { resolve, dirname } from "node:path";
import { fileURLToPath } from "node:url";
import process from "node:process";

export function releaseVersion(root, ref = "") {
  const frontend = JSON.parse(readFileSync(resolve(root, "package.json"), "utf8")).version;
  const app = JSON.parse(readFileSync(resolve(root, "src-tauri/tauri.conf.json"), "utf8")).version;
  const cargo = readFileSync(resolve(root, "Cargo.toml"), "utf8");
  const workspace = cargo.match(/\[workspace\.package\]([\s\S]*?)(?=\n\[|$)/)?.[1];
  const rust = workspace?.match(/^version\s*=\s*"([^"]+)"\s*$/m)?.[1];
  if (!rust || frontend !== app || app !== rust) {
    throw new Error(`Versions must match: package.json=${frontend}, Tauri=${app}, Cargo=${rust ?? "missing"}`);
  }
  if (!/^\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?(?:\+[0-9A-Za-z.-]+)?$/.test(app)) {
    throw new Error(`Invalid release version: ${app}`);
  }
  const tag = `v${app}`;
  if (ref.startsWith("refs/tags/") && ref !== `refs/tags/${tag}`) {
    throw new Error(`Tag ${ref.slice("refs/tags/".length)} does not match application version ${tag}`);
  }
  return { version: app, tag };
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");
  const result = releaseVersion(root, process.env.GITHUB_REF);
  if (process.env.GITHUB_OUTPUT) {
    appendFileSync(process.env.GITHUB_OUTPUT, `version=${result.version}\ntag=${result.tag}\n`);
  }
  console.log(result.tag);
}
