---
type: Rust Function
title: crash_recovery_resumes_at_correct_task
resource: tests/plan_crash_recovery_test.rs#L69-L126
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/db/Database/run_migrations
  - functions/tests/plan_crash_recovery_test/create_plan
  - functions/src/db/repository/plan/PlanTaskRepository/create_task
  - functions/tests/plan_crash_recovery_test/minimal_task
---

# Signature

`async fn crash_recovery_resumes_at_correct_task()`

# Calls

- [run_migrations](../../../functions/src/db/Database/run_migrations.md)
- [create_plan](../../../functions/tests/plan_crash_recovery_test/create_plan.md)
- [create_task](../../../functions/src/db/repository/plan/PlanTaskRepository/create_task.md)
- [minimal_task](../../../functions/tests/plan_crash_recovery_test/minimal_task.md)