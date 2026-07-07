# CLAUDE.md

Guidance for Claude Code when working in this repository.

## graphify

This project has a knowledge graph at `docs/graph/` (graph.json, GRAPH_REPORT.md).

Rules:
- For codebase questions, first run `GRAPHIFY_OUT=docs/graph graphify query "<question>"`.
  Use `graphify path "<A>" "<B>" --graph docs/graph/graph.json` for relationships
  and `graphify explain "<concept>" --graph docs/graph/graph.json` for focused concepts.
  These return a scoped subgraph, usually much smaller than GRAPH_REPORT.md or raw grep output.
- Dirty `docs/graph/` files are expected after the post-commit hook or a manual
  update runs; that alone is not a reason to skip using the graph.
- Read `docs/graph/GRAPH_REPORT.md` for broad architecture questions (god nodes,
  cross-community connections, suggested questions) or when query/path/explain
  don't surface enough context.
- After modifying Rust code, the post-commit git hook already refreshes
  `docs/graph/` (AST-only, no API cost) - no manual rebuild needed. Docs and
  image changes still need a manual `/graphify --update .` pass since that
  requires an LLM. See `docs/graph/README.md` for details.
