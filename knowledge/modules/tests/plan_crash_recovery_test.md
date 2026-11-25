---
type: Rust Module
title: plan_crash_recovery_test
resource: tests/plan_crash_recovery_test.rs#L1-L236
generated:
  by: okf-rs/0.8.0
relationships:
  imports:
  - target: external/crustly-db-models-interrupted-plan-from-tasks-plantaskstatus
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-db-repository-plantaskrepository
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-db-database
    resolved_by: tree-sitter
    confidence: exact
  - target: external/uuid-uuid
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [create_session](../../functions/tests/plan_crash_recovery_test/create_session.md)
- [create_plan](../../functions/tests/plan_crash_recovery_test/create_plan.md)
- [minimal_task](../../functions/tests/plan_crash_recovery_test/minimal_task.md)
- [crash_recovery_resumes_at_correct_task](../../functions/tests/plan_crash_recovery_test/crash_recovery_resumes_at_correct_task.md)
- [task_state_transitions_correct_order](../../functions/tests/plan_crash_recovery_test/task_state_transitions_correct_order.md)
- [failed_task_stores_error_without_completion_timestamp](../../functions/tests/plan_crash_recovery_test/failed_task_stores_error_without_completion_timestamp.md)
- [interrupted_plan_none_when_all_done](../../functions/tests/plan_crash_recovery_test/interrupted_plan_none_when_all_done.md)
- [interrupted_plan_resumes_at_lowest_incomplete](../../functions/tests/plan_crash_recovery_test/interrupted_plan_resumes_at_lowest_incomplete.md)

# Imports

- `crustly::db::models::{interrupted_plan_from_tasks, PlanTaskStatus}`
- `crustly::db::repository::PlanTaskRepository`
- `crustly::db::Database`
- `uuid::Uuid`

# Member of

- [crustly](../../packages/crustly.md)