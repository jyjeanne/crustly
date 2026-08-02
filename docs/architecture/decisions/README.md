# Architecture Decision Records

The one thing in `docs/architecture/` that stays hand-written - per the
guide's principle "source code is the only authoritative source, everything
else is generated," ADRs are the explicit exception, because a generator
cannot know *why* a decision was made, only what the code currently looks
like.

## When to write one

A new ADR when a decision:

- is hard to reverse (a dependency choice that ripples through the codebase,
  a storage format, a protocol),
- was between two or more real alternatives, and
- future contributors will otherwise ask "wait, why did we do it this way?"

Not every code change needs one. A bug fix doesn't; "we replaced provider X
with Y because of a version conflict" does.

## How to write one

1. Copy `0000-adr-template.md` to `NNNN-short-title.md` (next sequence number,
   lowercase-kebab-case title).
2. Fill in Context / Decision / Consequences. Keep it short - a paragraph or
   two per section is usually enough.
3. Status starts as `Proposed`, moves to `Accepted` once agreed, and to
   `Superseded by ADR-NNNN` if a later decision replaces it. Never delete or
   rewrite an old ADR to reflect a new decision - record the new one and
   mark the old one superseded, so the history stays honest.

## Index

| ADR | Title | Status |
|---|---|---|
| [0001](0001-record-architecture-decisions.md) | Record architecture decisions | Accepted |
| [0002](0002-sqlx-over-rusqlite.md) | Use sqlx exclusively, not rusqlite | Accepted |
| [0003](0003-crabrace-provider-registry.md) | Crabrace for provider discovery (replaces Catwalk) | Accepted |
| [0004](0004-plan-mode-read-only-with-approval-gating.md) | Plan Mode: read-only planning with explicit approval gating | Accepted |
| [0005](0005-llama-cpp-in-process-worker-thread.md) | In-process llama.cpp provider: one dedicated OS worker thread per model | Accepted |
