#!/usr/bin/env bash
# Generates docs/architecture/repository.md from Cargo.toml + `cargo metadata`.
# Phase 1 of docs/architecture/PLAN.md - a cheap, regenerable crate/dependency
# summary. Re-run after adding/removing dependencies or bumping the version.
#
# Usage: scripts/generate-architecture-docs.sh

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"
mkdir -p docs/architecture

PYTHON="$(command -v python3 || command -v python)"
if [ -z "$PYTHON" ]; then
  echo "python3 not found" >&2
  exit 1
fi

META_JSON="$(cargo metadata --no-deps --format-version 1)"

"$PYTHON" - "$META_JSON" <<'PYEOF'
import json, re, subprocess, sys
from pathlib import Path

meta = json.loads(sys.argv[1])
pkg = meta["packages"][0]

# Group [dependencies] by the "# Category" comment lines above them, straight
# out of Cargo.toml - the file is already organized this way, no need to
# re-invent a categorization.
cargo_toml = Path("Cargo.toml").read_text(encoding="utf-8")
dep_section = cargo_toml.split("[dependencies]", 1)[1].split("[dev-dependencies]", 1)[0]

groups: list[tuple[str, list[str]]] = []
current_label = "Uncategorized"
current_deps: list[str] = []
prev_was_blank = True  # start of file counts as a fresh section
for raw_line in dep_section.splitlines():
    line = raw_line.strip()
    if not line:
        prev_was_blank = True
        continue
    # Only a comment immediately after a blank line is a category header;
    # a comment following another line (code or comment) is an inline note
    # (e.g. "# Note: ...", multi-line explanations) and must not start a
    # new section or be mistaken for one.
    m = re.match(r"^#\s*(.+)$", line)
    if m and prev_was_blank:
        if current_deps:
            groups.append((current_label, current_deps))
        current_label = m.group(1)
        current_deps = []
        prev_was_blank = False
        continue
    prev_was_blank = False
    if line.startswith("#"):
        continue
    m = re.match(r'^([A-Za-z0-9_-]+)\s*=', line)
    if m:
        current_deps.append(m.group(1))
if current_deps:
    groups.append((current_label, current_deps))

targets_by_kind: dict[str, list[str]] = {}
for t in pkg["targets"]:
    for kind in t["kind"]:
        targets_by_kind.setdefault(kind, []).append(t["name"])

lines = []
lines.append("# Repository Summary")
lines.append("")
lines.append("Generated from `Cargo.toml` + `cargo metadata` by "
              "`scripts/generate-architecture-docs.sh`. Do not hand-edit - "
              "re-run the script instead.")
lines.append("")
lines.append("## Crate")
lines.append("")
lines.append(f"- **Name:** {pkg['name']}")
lines.append(f"- **Version:** {pkg['version']}")
lines.append(f"- **Edition:** {pkg['edition']}")
lines.append(f"- **License:** {pkg.get('license', 'n/a')}")
lines.append(f"- **Rust edition:** {pkg['edition']}")
lines.append("")
lines.append("## Targets")
lines.append("")
for kind, names in targets_by_kind.items():
    lines.append(f"- **{kind}** ({len(names)}): {', '.join(sorted(set(names)))}")
lines.append("")
lines.append("## Features")
lines.append("")
features = pkg.get("features", {})
for name, deps in sorted(features.items()):
    dep_str = f" -> {', '.join(deps)}" if deps else ""
    lines.append(f"- `{name}`{dep_str}")
lines.append("")
lines.append("## Dependencies (by category, from Cargo.toml)")
lines.append("")
for label, deps in groups:
    lines.append(f"### {label}")
    lines.append("")
    for d in deps:
        lines.append(f"- `{d}`")
    lines.append("")

Path("docs/architecture/repository.md").write_text("\n".join(lines) + "\n", encoding="utf-8")
print("docs/architecture/repository.md written")
PYEOF
