---
type: Rust Function
title: render_plan_document
resource: src/tui/render.rs#L1525-L1615
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/tool_round/ToolRound/push
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_plan_task_lines
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/render/render_plan
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn render_plan_document(plan: &crate::plan::PlanDocument, area_width: usize) -> Vec<Line<'_>>`

# Calls

- [push](../../../../functions/src/llm/agent/service/tool_round/ToolRound/push.md)
- [render_plan_task_lines](../../../../functions/src/tui/render/render_plan_task_lines.md)

# Called by

- [render_plan](../../../../functions/src/tui/render/render_plan.md)