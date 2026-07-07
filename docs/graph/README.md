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

## Using this graph with Claude Code

The graph lives at `docs/graph/graph.json`, not graphify's default
`graphify-out/graph.json`, so point every command at it: either pass
`--graph docs/graph/graph.json` or export `GRAPHIFY_OUT=docs/graph` once per
shell so the default path resolves correctly.

Ask Claude Code a question in plain English (e.g. "how does the TUI talk to
the LLM provider?", "what would break if I changed the `Provider` trait?")
and it can answer from the graph instead of grepping the whole repo - either
by running the CLI itself or by reading `docs/graph/graph.json` directly:

```bash
# broad question -> BFS traversal of the graph (cheap, scoped context)
GRAPHIFY_OUT=docs/graph graphify query "How does the TUI talk to the LLM provider?"

# relationship between two named things -> shortest path
graphify path "AgentService" "SqlitePool" --graph docs/graph/graph.json

# focused explanation of one node and its neighbors
graphify explain "Provider" --graph docs/graph/graph.json

# what would be affected by changing X (reverse traversal)
graphify affected "AgentService" --graph docs/graph/graph.json
```

For architecture-level questions ("what are the core abstractions?", "what's
surprising about this codebase?"), point Claude Code at
`docs/graph/GRAPH_REPORT.md` instead - it already has the god nodes,
cross-community "surprising connections," and suggested questions written up.

### Making it automatic

The repo root already has `CLAUDE.md` and `AGENTS.md`, each with a `## graphify`
section instructing the agent to check `docs/graph/` before answering codebase
questions - Claude Code reads `CLAUDE.md`; Codex, Aider, OpenCode, Factory
Droid, Trae, and OpenClaw read `AGENTS.md`. No per-session setup needed;
update those files if the graph's location or workflow changes.
