#!/usr/bin/env bash
# One-time setup: point git at the repo-tracked hooks in .githooks/ so
# docs/graph auto-updates after commits/checkouts that touch Rust code.
# Run this once after cloning: scripts/setup-graphify-hooks.sh

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

if ! command -v graphify >/dev/null 2>&1; then
  echo "[graphify] 'graphify' CLI not found - installing via uv..."
  if command -v uv >/dev/null 2>&1; then
    uv tool install graphifyy
  else
    echo "[graphify] 'uv' not found either. Install one of:" >&2
    echo "    uv tool install graphifyy" >&2
    echo "    pipx install graphifyy" >&2
    exit 1
  fi
fi

chmod +x .githooks/post-commit .githooks/post-checkout
git config core.hooksPath .githooks

echo "[graphify] git hooks installed (core.hooksPath = .githooks)"
echo "[graphify] docs/graph will auto-update and auto-commit after commits touching *.rs/Cargo.toml/Cargo.lock"
echo "[graphify] to disable temporarily: GRAPHIFY_SKIP_HOOK=1 git commit ..."
