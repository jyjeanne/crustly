# 0004. Plan Mode: read-only planning with explicit approval gating

Status: Accepted

## Context

Letting an LLM agent execute multi-step tasks directly against a user's
codebase is risky without a review step - a bad plan executed immediately is
harder to recover from than a bad plan caught before any tool runs. Crustly
needed a way to let the model explore and propose a multi-step plan without
being able to write, using a different model for planning vs. execution if
desired, and giving the user a checkpoint before anything happens for real.

## Decision

Plan Mode: the agent explores the codebase read-only (tool capabilities are
filtered by `AppMode` - write-capable tools are rejected while
`app_mode == AppMode::Plan`), produces a structured `PlanDocument` /
`PlanTask` list, and executes only after explicit user approval. See
`docs/development/PLAN_MODE_DESIGN.md` and `docs/PLAN_MODE_USER_GUIDE.md` for
the full design and user-facing walkthrough.

## Consequences

Safer execution, clearer task tracking, and the flexibility to route planning
and execution to different models. One cost, found during Phase 3 module-
coupling analysis (`docs/architecture/module-coupling.md`): `PlanDocument`,
`PlanTask`, and related types were originally defined in `src/tui/plan.rs` -
the UI layer - because Plan Mode's UI was built first and the data model
followed it. `src/db/repository/plan.rs`, `src/llm/tools/plan_tool.rs`, and
`src/services/plan.rs` all depended on the UI module for their own domain
types, inverting the intended layering (`docs/architecture/containers.md`).
`PlanDocument` is also a top god node in `docs/graph/GRAPH_REPORT.md`.

**Resolved:** the Plan domain model now lives in `src/plan/` at the crate
root, dependency-free (its one DB-consuming constructor moved to
`crate::db::models::interrupted_plan_from_tasks`, keeping the db -> plan
direction). `tui`, `db`, `llm`, and `services` all depend on `crate::plan`;
no lower layer depends on `tui`. The invariant to preserve: `src/plan/` must
stay free of `db`, `tui`, and every other crate-internal dependency - the
coupling report's layering check will flag regressions.
