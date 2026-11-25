---
type: Rust Method
title: save_plan
resource: src/tui/app.rs#L2981-L3018
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/services/plan/PlanService/update
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/plan/PlanService/create
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/plan/PlanService/export_to_json
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/app/App/handle_plan_key
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/check_task_completion
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/execute_next_plan_task
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/fail_current_plan_task
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn save_plan(&self) -> Result<()>`

# Calls

- [update](../../../../../functions/src/services/plan/PlanService/update.md)
- [create](../../../../../functions/src/services/plan/PlanService/create.md)
- [export_to_json](../../../../../functions/src/services/plan/PlanService/export_to_json.md)

# Called by

- [handle_plan_key](../../../../../functions/src/tui/app/App/handle_plan_key.md)
- [check_task_completion](../../../../../functions/src/tui/app/App/check_task_completion.md)
- [execute_next_plan_task](../../../../../functions/src/tui/app/App/execute_next_plan_task.md)
- [fail_current_plan_task](../../../../../functions/src/tui/app/App/fail_current_plan_task.md)