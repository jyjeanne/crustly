---
type: Rust Function
title: task_state_transitions_correct_order
resource: tests/plan_crash_recovery_test.rs#L130-L171
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
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
  - target: functions/tests/plan_crash_recovery_test/minimal_task
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/plan/PlanTaskRepository/create_task
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/plan/PlanTaskRepository/update_task_status
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/plan/PlanTaskRepository/get_task
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/models/PlanTask/exec_status
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn task_state_transitions_correct_order()`

# Calls

- [connect_in_memory](../../../functions/src/db/Database/connect_in_memory.md)
- [run_migrations](../../../functions/src/db/Database/run_migrations.md)
- [pool](../../../functions/src/db/Database/pool.md)
- [create_plan](../../../functions/tests/plan_crash_recovery_test/create_plan.md)
- [new](../../../functions/src/db/repository/plan/PlanTaskRepository/new.md)
- [minimal_task](../../../functions/tests/plan_crash_recovery_test/minimal_task.md)
- [create_task](../../../functions/src/db/repository/plan/PlanTaskRepository/create_task.md)
- [update_task_status](../../../functions/src/db/repository/plan/PlanTaskRepository/update_task_status.md)
- [get_task](../../../functions/src/db/repository/plan/PlanTaskRepository/get_task.md)
- [exec_status](../../../functions/src/db/models/PlanTask/exec_status.md)