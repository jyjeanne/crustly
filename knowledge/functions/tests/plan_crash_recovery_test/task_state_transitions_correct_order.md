---
type: Rust Function
title: task_state_transitions_correct_order
resource: tests/plan_crash_recovery_test.rs#L130-L171
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/db/Database/run_migrations
  - functions/tests/plan_crash_recovery_test/create_plan
  - functions/tests/plan_crash_recovery_test/minimal_task
  - functions/src/db/repository/plan/PlanTaskRepository/create_task
  - functions/src/db/repository/plan/PlanTaskRepository/update_task_status
---

# Signature

`async fn task_state_transitions_correct_order()`

# Calls

- [run_migrations](../../../functions/src/db/Database/run_migrations.md)
- [create_plan](../../../functions/tests/plan_crash_recovery_test/create_plan.md)
- [minimal_task](../../../functions/tests/plan_crash_recovery_test/minimal_task.md)
- [create_task](../../../functions/src/db/repository/plan/PlanTaskRepository/create_task.md)
- [update_task_status](../../../functions/src/db/repository/plan/PlanTaskRepository/update_task_status.md)