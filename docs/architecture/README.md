# Architecture Documentation

Phase 1 of applying the "Architecture Documentation Generation" methodology to
crustly (see the analysis in the session that created this - a full multi-tool
pipeline would be overkill for a single-crate CLI project already served by
[graphify](../graph/README.md); this directory covers the cheap, high-value
slice: crate/dependency facts and C4-style context/container diagrams).

- `repository.md` - generated from `Cargo.toml` + `cargo metadata` by
  `scripts/generate-architecture-docs.sh`. Re-run the script after changing
  dependencies or bumping the version; do not hand-edit.
- `context.md` - C4 System Context (L1), hand-authored, Mermaid.
- `containers.md` - C4 Containers (L2), hand-authored, Mermaid, mirrors
  `docs/ARCHITECTURE.md` §1.

## What this is not (yet)

This is deliberately a small slice, not the full guide:

- No automated Component (L3) diagrams, cycle/orphan-module validation, or
  `cargo-modules`/`cargo-machete` checks yet - deferred to a later phase if
  wanted.
- No Structurizr/LikeC4, Neo4j/Memgraph, Gephi, or PlantUML/D2 - graphify's
  own `graph.json` + CLI (`query`/`path`/`explain`) already cover graph
  exploration at this project's scale; adding a second graph database or a
  parallel architecture-as-code DSL would just be two things to keep in sync
  instead of one.
- `docs/ARCHITECTURE.md` remains the detailed narrative; `containers.md` is
  its GitHub-renderable summary, not a replacement.

See [`docs/graph/README.md`](../graph/README.md) for the AI-facing knowledge
graph and its automation, which already covers most of what the guide calls
the "Symbol Graph" / "AI Documents" layers.
