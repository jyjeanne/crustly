---
type: Rust Method
title: check_and_load_plan
resource: src/tui/app.rs#L2747-L2897
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/services/plan/PlanService/get_most_recent_plan
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/CodeKnowledge/exists
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/plan/PlanService/create
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/skill/SkillSource/display
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/app/App/complete_response
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn check_and_load_plan(&mut self) -> Result<()>`

# Calls

- [get_most_recent_plan](../../../../../functions/src/services/plan/PlanService/get_most_recent_plan.md)
- [exists](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/exists.md)
- [create](../../../../../functions/src/services/plan/PlanService/create.md)
- [display](../../../../../functions/src/llm/tools/skill/SkillSource/display.md)

# Called by

- [complete_response](../../../../../functions/src/tui/app/App/complete_response.md)