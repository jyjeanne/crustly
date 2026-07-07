# AGENTS.md

Guidance for AI coding agents (Codex, Aider, OpenCode, Factory Droid, Trae,
OpenClaw, and similar) working in this repository.

## graphify

This project has a knowledge graph at `docs/graph/` (graph.json, GRAPH_REPORT.md).

When the user types `/graphify`, use the installed graphify skill or
instructions before doing anything else.

Rules:
- For codebase questions, first run `GRAPHIFY_OUT=docs/graph graphify query "<question>"`.
  Use `graphify path "<A>" "<B>" --graph docs/graph/graph.json` for relationships
  and `graphify explain "<concept>" --graph docs/graph/graph.json` for focused concepts.
  These return a scoped subgraph, usually much smaller than GRAPH_REPORT.md or raw grep output.
- Dirty `docs/graph/` files are expected after the post-commit hook or an
  incremental update; that alone is not a reason to skip using the graph.
  Only skip it if the task is specifically about stale/incorrect graph output,
  or the user explicitly says not to use it.
- Read `docs/graph/GRAPH_REPORT.md` only for broad architecture review or when
  query/path/explain do not surface enough context.
- After modifying Rust code, a git post-commit hook already refreshes
  `docs/graph/` (AST-only, no API cost) - no manual rebuild needed for code
  changes. Docs/image changes still need a manual `graphify extract .` or
  `/graphify --update .` pass since that requires an LLM. See
  `docs/graph/README.md` for details.
