---
type: Rust Module
title: plan_autorun_test
resource: tests/plan_autorun_test.rs#L1-L141
generated:
  by: okf-rs/0.3.0
relationships:
  imports:
  - external/crustly-plan-autorunmode-planmodestate-plantask-taskstatus-tasktype
  - external/uuid-uuid
  member_of:
  - packages/crustly
---

# Contains

- [make_plan_task](../../functions/tests/plan_autorun_test/make_plan_task.md)
- [auto_plan_approval_goes_to_auto_executing](../../functions/tests/plan_autorun_test/auto_plan_approval_goes_to_auto_executing.md)
- [interactive_approval_goes_to_executing](../../functions/tests/plan_autorun_test/interactive_approval_goes_to_executing.md)
- [high_risk_tools_pause_auto_execution](../../functions/tests/plan_autorun_test/high_risk_tools_pause_auto_execution.md)
- [advance_transitions_through_tasks_to_done](../../functions/tests/plan_autorun_test/advance_transitions_through_tasks_to_done.md)

# Imports

- `crustly::plan::{AutoRunMode, PlanModeState, PlanTask, TaskStatus, TaskType}`
- `uuid::Uuid`

# Member of

- [crustly](../../packages/crustly.md)