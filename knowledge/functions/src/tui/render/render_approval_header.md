---
type: Rust Function
title: render_approval_header
resource: src/tui/render.rs#L1710-L1762
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/events/ToolApprovalRequest/time_remaining
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/render/render_approval
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn render_approval_header<'a>( request: &'a super::events::ToolApprovalRequest, model_name: &'a str, ) -> Vec<Line<'a>>`

# Calls

- [time_remaining](../../../../functions/src/tui/events/ToolApprovalRequest/time_remaining.md)

# Called by

- [render_approval](../../../../functions/src/tui/render/render_approval.md)