---
type: Rust Function
title: render_approval_input_summary
resource: src/tui/render.rs#L1828-L1867
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/tool_round/ToolRound/push
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/utils/truncate_at_char_boundary
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/render/render_approval
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn render_approval_input_summary(request: &super::events::ToolApprovalRequest) -> Vec<Line<'_>>`

# Calls

- [push](../../../../functions/src/llm/agent/service/tool_round/ToolRound/push.md)
- [truncate_at_char_boundary](../../../../functions/src/utils/truncate_at_char_boundary.md)

# Called by

- [render_approval](../../../../functions/src/tui/render/render_approval.md)