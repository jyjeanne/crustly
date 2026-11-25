---
type: Rust Method
title: create
resource: src/services/plan.rs#L108-L110
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/db/repository/plan/PlanRepository/create
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/tui/app/App/load_plan_for_viewing
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/app/App/check_and_load_plan
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/app/App/save_plan
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn create(&self, plan: &PlanDocument) -> Result<()>`

# Calls

- [create](../../../../../functions/src/db/repository/plan/PlanRepository/create.md)

# Called by

- [load_plan_for_viewing](../../../../../functions/src/tui/app/App/load_plan_for_viewing.md)
- [check_and_load_plan](../../../../../functions/src/tui/app/App/check_and_load_plan.md)
- [save_plan](../../../../../functions/src/tui/app/App/save_plan.md)