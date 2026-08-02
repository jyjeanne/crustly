---
type: Rust Function
title: failed_task_stores_error_without_completion_timestamp
resource: tests/plan_crash_recovery_test.rs#L175-L208
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/db/Database/run_migrations
  - functions/tests/plan_crash_recovery_test/create_plan
  - functions/tests/plan_crash_recovery_test/minimal_task
  - functions/src/db/repository/plan/PlanTaskRepository/create_task
  - functions/src/db/repository/plan/PlanTaskRepository/update_task_status
---

# Signature

`async fn failed_task_stores_error_without_completion_timestamp()`

# Calls

- [run_migrations](../../../functions/src/db/Database/run_migrations.md)
- [create_plan](../../../functions/tests/plan_crash_recovery_test/create_plan.md)
- [minimal_task](../../../functions/tests/plan_crash_recovery_test/minimal_task.md)
- [create_task](../../../functions/src/db/repository/plan/PlanTaskRepository/create_task.md)
- [update_task_status](../../../functions/src/db/repository/plan/PlanTaskRepository/update_task_status.md)