#!/usr/bin/env python3
"""Regenerate THIRD_PARTY_LICENSES.md from `cargo metadata` (Windows + macOS targets).

Usage: python3 scripts/gen-licenses.py
"""
import json
import pathlib
import subprocess

ROOT = pathlib.Path(__file__).resolve().parent.parent
OWN = {"knorozov-core", "knorozov-cli", "knorozov-pro"}
BLOCKED = ("GPL", "AGPL", "LGPL", "CC-BY-NC", "SSPL")

crates = {}
for target in ("x86_64-pc-windows-msvc", "aarch64-apple-darwin"):
    meta = json.loads(
        subprocess.check_output(
            ["cargo", "metadata", "--format-version", "1", "--filter-platform", target], cwd=ROOT
        )
    )
    ids = {n["id"] for n in meta["resolve"]["nodes"]}
    for p in meta["packages"]:
        if p["id"] in ids and p["name"] not in OWN:
            crates[(p["name"], p["version"])] = (p.get("license") or "см. репозиторий", p.get("repository") or "")

bad = [(n, v, l) for (n, v), (l, _) in crates.items() if any(b in l for b in BLOCKED) and "OR" not in l]
if bad:
    raise SystemExit(f"Blocked licenses found: {bad}")

head = (ROOT / "scripts" / "licenses-header.md").read_text(encoding="utf-8")
lines = [head, "\n## Rust crates (автоматически, `scripts/gen-licenses.py`)\n", "| Crate | Version | License |", "|---|---|---|"]
for (n, v), (l, repo) in sorted(crates.items()):
    name = f"[{n}]({repo})" if repo else n
    lines.append(f"| {name} | {v} | {l} |")
(ROOT / "THIRD_PARTY_LICENSES.md").write_text("\n".join(lines) + "\n", encoding="utf-8")
print(f"wrote THIRD_PARTY_LICENSES.md with {len(crates)} crates")
