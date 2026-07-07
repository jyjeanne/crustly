# 0001. Record architecture decisions

Status: Accepted

## Context

Most of `docs/architecture/` and `docs/graph/` is generated from source and
answers *what* the codebase looks like. Neither can answer *why* a
hard-to-reverse choice was made - that context lives in scattered docs
(`docs/guides/`, `docs/development/`) or nowhere at all, and is the first
thing lost when the person who made the call moves on.

## Decision

We use lightweight Architecture Decision Records (ADRs), one file per
decision, numbered sequentially, in `docs/architecture/decisions/`. See the
directory's `README.md` for when to write one.

## Consequences

Decisions that matter get a permanent, dated record instead of living only in
a PR description or a Discord thread. The cost is discipline: an ADR that
never gets written because "we'll do it later" is worth exactly as much as no
ADR at all.
