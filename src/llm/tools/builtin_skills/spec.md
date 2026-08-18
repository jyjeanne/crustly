---
name: spec
description: Specification-Driven Development workflow - turns a feature description into a versioned spec, plan, and task list under specs/, then builds it natively (no external agent handoff) and validates the result. Advance phases with trailing args (e.g. "/spec plan", "/spec tasks", "/spec implement", "/spec analyze") or start a new feature with a description.
---

# Spec

Insert a structured specification layer between an idea and its code, so
scope, edge cases, and traceability survive contact with an AI agent. Every
feature gets `specs/<NNN>-<slug>/spec.md` → `plan.md` → `tasks.md`, all
version-controlled, all driving what actually gets built.

## Determine phase and target feature

The trailing argument (after "Additional arguments:") decides what happens:

- **A phase word** (`plan`, `tasks`, `implement`, `analyze`) with no feature
  number — advance the current feature (the one on the current git branch,
  or the highest-numbered `specs/NNN-*` directory) to that phase.
- **A phase word plus a number** (`plan 003`) — advance that specific feature.
- **Anything else, or no active feature exists yet** — treat the whole
  argument as a new feature description and start at Step 1 (Specify).
- **No argument and no active feature** — ask what feature to specify; don't
  guess.

Never skip a phase whose artifact doesn't exist yet: `tasks` requires
`plan.md`, `implement` requires `tasks.md`, `analyze` requires all three.
If the prerequisite is missing, say so and run that phase first instead of
proceeding on stale or absent context.

## Step 1: Specify

1. Use `bash`/`glob` to find the next feature number: highest existing
   `specs/NNN-*` + 1, zero-padded to 3 digits (`001` if none exist).
2. Derive a short kebab-case slug from the description (3-5 words).
3. Write `specs/<NNN>-<slug>/spec.md` containing:
   - **User stories**, each tagged `P1`/`P2`/`P3` by priority, written as
     "As a ..., I want ..., so that ...".
   - **Functional requirements**, numbered `FR-001`, `FR-002`, ... — each
     one sentence, each testable.
   - **Acceptance scenarios** per story, in Given/When/Then form.
   - Any genuine ambiguity in the description gets a `[NEEDS CLARIFICATION:
     specific question]` marker inline — do not silently guess at
     unstated scope, but don't manufacture markers for things a reasonable
     default answers.
   - A short quality checklist: no vague terms ("fast", "user-friendly")
     left unquantified, no requirement without a matching acceptance
     scenario.
4. If `[NEEDS CLARIFICATION]` markers exist, list them and ask the user to
   resolve them before moving on — resolving them means editing `spec.md`
   directly (a full separate clarifications log is unnecessary overhead
   for most features; add one only if the project already keeps one).

Stop here. The user reviews `spec.md` before `plan` runs against it.

## Step 2: Plan (`plan` argument)

1. Read `spec.md`. If unresolved `[NEEDS CLARIFICATION]` markers remain,
   stop and say which ones block planning.
2. Read the project's own `CLAUDE.md`/`AGENTS.md` (root and any relevant
   subdirectory) the same way the `review` skill does — the architecture
   this plan produces must fit the existing codebase, not a generic one.
3. Write `specs/<NNN>-<slug>/plan.md` with:
   - **Architecture summary**: what modules/files are added or changed,
     and why, in terms of the existing module layout.
   - **Data model** (if the feature has one): entities, fields,
     relationships — inline as a section, not a separate file.
   - **Constitution check** — a short gate before anything gets built:
     - *Simplicity*: does this need a new abstraction layer, or does an
       existing module already do most of this?
     - *Anti-abstraction*: are existing crates/tools used directly, or is
       this wrapping something that didn't need wrapping?
     - *Integration-first*: can this be tested against the real
       tool/service it touches, or does it only work against a mock?
     Gate violations are reported, not silently fixed and not blocking —
     note them in `plan.md` and let the user decide whether to proceed.
4. If the plan reveals the spec itself was wrong or incomplete, say so and
   propose the spec edit — don't quietly plan around a bad requirement.

## Step 3: Tasks (`tasks` argument)

1. Read `plan.md`. Break the work into phases:
   - **Setup** — anything needed before any story can be built.
   - **Foundational** — shared code every story depends on.
   - One phase **per user story**, ordered by priority (P1 first).
2. Write `specs/<NNN>-<slug>/tasks.md` as a checklist, each task:
   - Numbered `T001`, `T002`, ...
   - Tagged `[US1]`/`[US2]`/... with the story it serves, and `[P]` when
     it can run in parallel with siblings in its phase (touches disjoint
     files, no shared state).
   - Small enough to review in one sitting — split anything that would
     touch more than a handful of files or mix unrelated concerns.
   - Referencing the `FR-###` it satisfies, so `analyze` can trace it back.

## Step 4: Implement (`implement` argument)

Build it directly in this session — there is no separate agent to hand off
to. Use the `task_manager` tool to track progress against `tasks.md`, and
`write_file`/`edit_file`/`bash` to make the changes:

1. Work phase by phase, in the order `tasks.md` lists them. Within a
   phase, tasks marked `[P]` may be done in either order; do not
   parallelize tasks that aren't marked `[P]` — they share state or files.
2. After each task (or each small batch of `[P]` tasks in the same area),
   run the project's own test/check commands (see `CLAUDE.md`) before
   moving on — a task isn't done because the code was written, it's done
   because it still compiles and passes.
3. Edit `tasks.md` to check off each completed task (`[ ]` → `[X]`) as you
   go, so `tasks.md` stays an honest progress record, not just a plan.
4. If a task turns out to need something the plan didn't anticipate, note
   the deviation in `tasks.md` next to that task rather than silently
   diverging from the written plan.

## Step 5: Analyze (`analyze` argument)

Read-only — this validates, it does not edit `spec.md`/`plan.md`/`tasks.md`.

1. Trace every `FR-###` in `spec.md` to a task in `tasks.md` that
   implements it, and every task back to the requirement it serves.
   Flag requirements with no task, and tasks with no requirement.
2. Check `tasks.md` completion against what's actually in the working
   tree — a task checked `[X]` whose described change isn't present is a
   finding, not a formality.
3. Re-run the project's tests; report pass/fail, not just "should pass."
4. Write `specs/<NNN>-<slug>/analysis-report.md` with the findings above
   plus a closing verdict: **READY** (traceable, tested, no open
   `[NEEDS CLARIFICATION]` markers) or **NEEDS WORK** (list exactly what,
   file:line where relevant) — the same bar the `review` skill's Step 6
   report applies to a diff.
