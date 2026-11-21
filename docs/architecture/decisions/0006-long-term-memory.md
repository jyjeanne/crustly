# 0006. Long-term memory: two scopes in SQLite, recall before each request

Status: Accepted

## Context

Crustly remembers only what the model writes to `.crustly/memory.md` through
`save_memory`, a file pasted into every system prompt. Nothing follows the
user from one repository to another, nothing can be searched, and the only way
to inspect or remove a memory is to edit the file. `save_memory` also needs no
approval, so a model steered by fetched content can plant an instruction that
is replayed in later sessions.

The episodic-memory code (`episodic_memories`, its repository and the
summary functions) was never called outside its tests, and the codebase index
was never started. Both were removed; their tables stay.

The full design, with alternatives considered, is in
`docs/improve-memory.md`. The alternatives that were rejected include a
separate store per memory type, a knowledge graph of memories, a reranking
model by default, and model-reported confidence scores. The code-structure
graph comes from okf-rs instead.

## Decision

We will build long-term memory on the existing SQLite database, with these
choices:

1. **Two scopes.** Global memories (the user's preferences) apply to every
   repository. Project memories stay in the repository they came from. Recall
   filters on scope in SQL, so no score can bring in another project's memory.
2. **A project is identified by its repository root**, hashed from the
   canonical path. The remote URL is display-only, so two clones of one remote
   are two projects.
3. **Memories are data, not instructions.** Project notes keep their
   instruction framing. Recalled memories go in a separate block, marked as
   possibly out of date, placed with the current user message and never stored
   in the history.
4. **`save_memory` writes to the database** when long-term memory is on.
   Model-saved global memories need approval in Interactive and AutoPlan modes.
   Importing the existing `.crustly/memory.md` bullets moves them, after a
   backup.
5. **Recall is keyword-first (FTS5)**, with optional local embeddings added in a
   later phase. Vectors are searched in Rust by brute force, not through a
   SQLite extension.
6. **Automatic extraction is opt-in and off by default**, and comes after
   recall is proven.
7. **Memories survive `crustly db clear`.** Deletion removes content from the
   database, the FTS index and the WAL (secure delete, then a WAL checkpoint).
8. **Three model-facing tools:** `save_memory` (extended), `search_memory` and
   `forget_memory`. Recall does not depend on tool calls, so small models that
   see only the core tools still get memory.
9. **The embedding client uses plain `reqwest`**, not the SSRF-guarded client,
   which blocks localhost by design.
10. **Stored hashes use SHA-256.** `DefaultHasher` is not stable across Rust
    releases and is never persisted.
11. **Dead code is removed; its tables are kept.** The episodic and codebase
    index code is deleted. The `episodic_memories` and `codebase_index` tables
    stay, because dropping a table is an irreversible migration. The
    `episodic_budget_tokens`, `enable_codebase_index` and `compaction_threshold`
    settings are kept so old config files load, and are documented as having no
    effect.
12. **Code knowledge comes from okf-rs** through its MCP server, when it is
    installed and the project has a bundle. Crustly does not parse source code
    for memory purposes.
13. **Migrations are never edited after merge**, and each phase's schema
    accepts the values later phases need, because SQLite cannot change a
    `CHECK` constraint without rebuilding the table.

## Consequences

Easier:

- Preferences carry over to every project. Project facts stay in their project.
- Memory is inspectable and removable from the CLI, and every save is announced.
- Keyword recall needs no model, so it works offline and on small models.
- Existing databases and config files keep loading unchanged.

Harder or accepted costs:

- Each phase is a schema migration. Migrations are forward-only, and the
  `CHECK` lists must be planned ahead.
- Keyword recall misses paraphrases. Semantic recall waits for Phase 2.
- Model-saved global memories interrupt the user with an approval prompt, which
  costs attention.
- Two clones of one remote do not share memory, by design.
- Unused config keys stay in the code until a future release removes them.
- Full deletion depends on SQLite secure delete and FTS5 secure-delete. Both
  are checked by a test, but the WAL file is only truncated by an explicit
  checkpoint.
