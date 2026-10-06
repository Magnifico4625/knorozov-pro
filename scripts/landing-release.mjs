import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import process from "node:process";
import { installerManifest } from "../landing/src/release-data.js";

let input = "";
for await (const chunk of process.stdin) input += chunk;
const releases = JSON.parse(input);
if (!Array.isArray(releases)) throw new Error("Expected a GitHub release list");
const manifest = installerManifest(releases);
writeFileSync(fileURLToPath(new URL("../landing/public/releases.json", import.meta.url)), `${JSON.stringify(manifest, null, 2)}\n`);
console.log(manifest.version ? `Download links: ${manifest.version}` : "No published installer release yet; the landing page will show an honest pending state.");
