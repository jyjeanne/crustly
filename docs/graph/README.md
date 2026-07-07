# Knowledge Graph

A [graphify](https://github.com/safishamsi/graphify)-generated knowledge graph of the
crustly codebase.

- `graph.html` - interactive graph, open in any browser
- `GRAPH_REPORT.md` - audit report (god nodes, surprising connections, suggested questions)
- `graph.json` - raw graph data, queryable with `graphify query "<question>"`

## Keeping it up to date

**Code (automatic):** after running `scripts/setup-graphify-hooks.sh` once, a git
post-commit hook re-extracts changed Rust files (AST only, no LLM, free) and
commits the result to `docs/graph/` whenever a commit touches `*.rs`,
`Cargo.toml`, or `Cargo.lock`. A post-checkout hook refreshes it (without
committing) when you switch branches.

To run it by hand: `scripts/update-knowledge-graph.sh`
To disable for one commit: `GRAPHIFY_SKIP_HOOK=1 git commit ...`

**Docs and images (manual):** the automated hook only covers Rust source - the
semantic pass over `docs/`, `README.md`, and screenshots needs an LLM. Refresh
that part periodically by running `/graphify --update .` in an AI assistant
session (Claude Code, etc.), then commit `docs/graph/`.
