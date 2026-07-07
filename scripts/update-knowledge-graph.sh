#!/usr/bin/env bash
# Rebuilds the crustly knowledge graph (Rust source only, no LLM) into docs/graph/
# using graphify (https://github.com/safishamsi/graphify). AST extraction is
# deterministic and free, so this is safe to run as often as needed.
#
# Docs and images are NOT covered here - that pass needs an LLM (Claude Code
# subagents or a configured API key) and is done via a manual `/graphify --update`
# run in an AI assistant session. See docs/graph/README.md.
#
# Usage:
#   scripts/update-knowledge-graph.sh            # rebuild only
#   scripts/update-knowledge-graph.sh --commit   # also commit docs/graph if it changed
#   scripts/update-knowledge-graph.sh --quiet    # suppress graphify's stdout

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

COMMIT=0
QUIET=0
for arg in "$@"; do
  case "$arg" in
    --commit) COMMIT=1 ;;
    --quiet) QUIET=1 ;;
  esac
done

if ! command -v graphify >/dev/null 2>&1; then
  echo "[graphify] 'graphify' CLI not found on PATH." >&2
  echo "  Install it with: uv tool install graphifyy   (or: pipx install graphifyy)" >&2
  exit 1
fi

export GRAPHIFY_OUT=docs/graph
mkdir -p "$GRAPHIFY_OUT"

if [ "$QUIET" = "1" ]; then
  graphify update . >/dev/null
else
  graphify update .
fi

if [ "$COMMIT" = "1" ]; then
  if [ -n "$(git status --porcelain -- "$GRAPHIFY_OUT")" ]; then
    git add "$GRAPHIFY_OUT"
    git commit -m "chore(graphify): update knowledge graph" --quiet
    echo "[graphify] committed updated $GRAPHIFY_OUT"
  fi
fi
