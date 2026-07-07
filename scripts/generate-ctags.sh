#!/usr/bin/env bash
# Generates a Universal Ctags symbol index at the repo root for editors
# (vim, emacs, etc). Not committed - regenerate after pulling or on demand.
#
# Usage: scripts/generate-ctags.sh

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

if ! command -v ctags >/dev/null 2>&1; then
  echo "[ctags] 'ctags' not found. Install Universal Ctags:" >&2
  echo "    Debian/Ubuntu: sudo apt-get install universal-ctags" >&2
  echo "    macOS:         brew install universal-ctags" >&2
  echo "    Windows:       choco install universal-ctags" >&2
  exit 1
fi

ctags --languages=Rust -R --exclude=target --exclude=.git --exclude=docs/graph -f tags src tests benches

echo "[ctags] wrote $(wc -l < tags) tags to ./tags"
