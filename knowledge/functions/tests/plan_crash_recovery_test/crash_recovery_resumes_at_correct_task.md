---
type: Rust Function
title: crash_recovery_resumes_at_correct_task
resource: tests/plan_crash_recovery_test.rs#L69-L126
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/Database/connect_in_memory
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/Database/run_migrations
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/Database/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/plan_crash_recovery_test/create_plan
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/plan/PlanTaskRepository/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/plan/PlanTaskRepository/create_task
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/plan_crash_recovery_test/minimal_task
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/plan/PlanTaskRepository/get_incomplete_tasks
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn crash_recovery_resumes_at_correct_task()`

# Calls

- [connect_in_memory](../../../functions/src/db/Database/connect_in_memory.md)
- [run_migrations](../../../functions/src/db/Database/run_migrations.md)
- [pool](../../../functions/src/db/Database/pool.md)
- [create_plan](../../../functions/tests/plan_crash_recovery_test/create_plan.md)
- [new](../../../functions/src/db/repository/plan/PlanTaskRepository/new.md)
- [create_task](../../../functions/src/db/repository/plan/PlanTaskRepository/create_task.md)
- [minimal_task](../../../functions/tests/plan_crash_recovery_test/minimal_task.md)
- [get_incomplete_tasks](../../../functions/src/db/repository/plan/PlanTaskRepository/get_incomplete_tasks.md)