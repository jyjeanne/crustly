---
type: Rust Method
title: update
resource: src/services/plan.rs#L113-L115
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/db/repository/plan/PlanRepository/update
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/tui/app/App/save_plan
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn update(&self, plan: &PlanDocument) -> Result<()>`

# Calls

- [update](../../../../../functions/src/db/repository/plan/PlanRepository/update.md)

# Called by

- [save_plan](../../../../../functions/src/tui/app/App/save_plan.md)