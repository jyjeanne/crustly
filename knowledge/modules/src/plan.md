---
type: Rust Module
title: plan
resource: src/plan/mod.rs#L1-L960
generated:
  by: okf-rs/0.3.0
relationships:
  imports:
  - external/chrono-datetime-utc
  - external/serde-deserialize-serialize
  - external/uuid-uuid
  - external/std-collections-hashmap-vecdeque
  member_of:
  - packages/crustly
---

# Contains

- [PlanDocument](../../classes/src/plan/PlanDocument.md)
- [new](../../functions/src/plan/PlanDocument/new.md)
- [add_task](../../functions/src/plan/PlanDocument/add_task.md)
- [tasks_in_order](../../functions/src/plan/PlanDocument/tasks_in_order.md)
- [get_task](../../functions/src/plan/PlanDocument/get_task.md)
- [get_task_mut](../../functions/src/plan/PlanDocument/get_task_mut.md)
- [count_by_status](../../functions/src/plan/PlanDocument/count_by_status.md)
- [progress_percentage](../../functions/src/plan/PlanDocument/progress_percentage.md)
- [is_complete](../../functions/src/plan/PlanDocument/is_complete.md)
- [approve](../../functions/src/plan/PlanDocument/approve.md)
- [reject](../../functions/src/plan/PlanDocument/reject.md)
- [start_execution](../../functions/src/plan/PlanDocument/start_execution.md)
- [complete](../../functions/src/plan/PlanDocument/complete.md)
- [validate_dependencies](../../functions/src/plan/PlanDocument/validate_dependencies.md)
- [next_executable_task](../../functions/src/plan/PlanDocument/next_executable_task.md)
- [next_executable_task_mut](../../functions/src/plan/PlanDocument/next_executable_task_mut.md)
- [get_task_by_order](../../functions/src/plan/PlanDocument/get_task_by_order.md)
- [get_task_by_order_mut](../../functions/src/plan/PlanDocument/get_task_by_order_mut.md)
- [dependencies_satisfied](../../functions/src/plan/PlanDocument/dependencies_satisfied.md)
- [execution_summary](../../functions/src/plan/PlanDocument/execution_summary.md)
- [ready_tasks](../../functions/src/plan/PlanDocument/ready_tasks.md)
- [retriable_tasks](../../functions/src/plan/PlanDocument/retriable_tasks.md)
- [get_validation_warnings](../../functions/src/plan/PlanDocument/get_validation_warnings.md)
- [ExecutionSummary](../../classes/src/plan/ExecutionSummary.md)
- [PlanStatus](../../classes/src/plan/PlanStatus.md)
- [fmt](../../functions/src/plan/PlanStatus/std-fmt-display/fmt.md)
- [PlanTask](../../classes/src/plan/PlanTask.md)
- [default_max_retries](../../functions/src/plan/default_max_retries.md)
- [TaskExecution](../../classes/src/plan/TaskExecution.md)
- [ToolCall](../../classes/src/plan/ToolCall.md)
- [new](../../functions/src/plan/PlanTask/new.md)
- [start](../../functions/src/plan/PlanTask/start.md)
- [start_execution](../../functions/src/plan/PlanTask/start_execution.md)
- [record_tool_call](../../functions/src/plan/PlanTask/record_tool_call.md)
- [complete_execution](../../functions/src/plan/PlanTask/complete_execution.md)
- [fail_execution](../../functions/src/plan/PlanTask/fail_execution.md)
- [add_reflection](../../functions/src/plan/PlanTask/add_reflection.md)
- [add_artifact](../../functions/src/plan/PlanTask/add_artifact.md)
- [can_retry](../../functions/src/plan/PlanTask/can_retry.md)
- [last_execution](../../functions/src/plan/PlanTask/last_execution.md)
- [complete](../../functions/src/plan/PlanTask/complete.md)
- [fail](../../functions/src/plan/PlanTask/fail.md)
- [block](../../functions/src/plan/PlanTask/block.md)
- [skip](../../functions/src/plan/PlanTask/skip.md)
- [complexity_stars](../../functions/src/plan/PlanTask/complexity_stars.md)
- [TaskType](../../classes/src/plan/TaskType.md)
- [fmt](../../functions/src/plan/TaskType/std-fmt-display/fmt.md)
- [TaskStatus](../../classes/src/plan/TaskStatus.md)
- [fmt](../../functions/src/plan/TaskStatus/std-fmt-display/fmt.md)
- [icon](../../functions/src/plan/TaskStatus/icon.md)
- [AutoRunMode](../../classes/src/plan/AutoRunMode.md)
- [PauseReason](../../classes/src/plan/PauseReason.md)
- [PlanModeState](../../classes/src/plan/PlanModeState.md)
- [tool_needs_approval](../../functions/src/plan/PlanModeState/tool_needs_approval.md)
- [approve](../../functions/src/plan/PlanModeState/approve.md)
- [advance](../../functions/src/plan/PlanModeState/advance.md)
- [is_high_risk_tool](../../functions/src/plan/PlanModeState/is_high_risk_tool.md)
- [InterruptedPlan](../../classes/src/plan/InterruptedPlan.md)

# Imports

- `chrono::{DateTime, Utc}`
- `serde::{Deserialize, Serialize}`
- `uuid::Uuid`
- `std::collections::{HashMap, VecDeque}`

# Member of

- [crustly](../../packages/crustly.md)