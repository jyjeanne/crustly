---
type: Rust Method
title: save_plan
resource: src/tui/app.rs#L1872-L1909
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/services/plan/PlanService/export_to_json
  called_by:
  - functions/src/tui/app/App/handle_plan_key
  - functions/src/tui/app/App/check_task_completion
  - functions/src/tui/app/App/execute_next_plan_task
  - functions/src/tui/app/App/fail_current_plan_task
---

# Signature

`async fn save_plan(&self) -> Result<()>`

# Calls

- [export_to_json](../../../../../functions/src/services/plan/PlanService/export_to_json.md)

# Called by

- [handle_plan_key](../../../../../functions/src/tui/app/App/handle_plan_key.md)
- [check_task_completion](../../../../../functions/src/tui/app/App/check_task_completion.md)
- [execute_next_plan_task](../../../../../functions/src/tui/app/App/execute_next_plan_task.md)
- [fail_current_plan_task](../../../../../functions/src/tui/app/App/fail_current_plan_task.md)