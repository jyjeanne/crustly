---
type: Rust Function
title: render_approval_input_detailed
resource: src/tui/render.rs#L1806-L1824
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/tool_round/ToolRound/push
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/render/render_approval
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn render_approval_input_detailed(request: &super::events::ToolApprovalRequest) -> Vec<Line<'_>>`

# Calls

- [push](../../../../functions/src/llm/agent/service/tool_round/ToolRound/push.md)

# Called by

- [render_approval](../../../../functions/src/tui/render/render_approval.md)