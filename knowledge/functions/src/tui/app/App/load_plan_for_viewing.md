---
type: Rust Method
title: load_plan_for_viewing
resource: src/tui/app.rs#L2674-L2742
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/services/plan/PlanService/get_most_recent_plan
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/plan/PlanService/create
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/skill/SkillSource/display
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/app/App/handle_key_event
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn load_plan_for_viewing(&mut self) -> Result<()>`

# Calls

- [get_most_recent_plan](../../../../../functions/src/services/plan/PlanService/get_most_recent_plan.md)
- [create](../../../../../functions/src/services/plan/PlanService/create.md)
- [display](../../../../../functions/src/llm/tools/skill/SkillSource/display.md)

# Called by

- [handle_key_event](../../../../../functions/src/tui/app/App/handle_key_event.md)