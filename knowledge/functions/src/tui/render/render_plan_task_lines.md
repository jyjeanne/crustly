---
type: Rust Function
title: render_plan_task_lines
resource: src/tui/render.rs#L1486-L1521
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/tool_round/ToolRound/push
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/PlanTask/complexity_stars
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/render/render_plan_document
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn render_plan_task_lines(task: &crate::plan::PlanTask, idx: usize) -> Vec<Line<'_>>`

# Calls

- [push](../../../../functions/src/llm/agent/service/tool_round/ToolRound/push.md)
- [complexity_stars](../../../../functions/src/plan/PlanTask/complexity_stars.md)

# Called by

- [render_plan_document](../../../../functions/src/tui/render/render_plan_document.md)