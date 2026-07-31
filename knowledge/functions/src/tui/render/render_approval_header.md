---
type: Rust Function
title: render_approval_header
resource: src/tui/render.rs#L1227-L1279
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/tui/events/ToolApprovalRequest/time_remaining
  called_by:
  - functions/src/tui/render/render_approval
---

# Signature

`fn render_approval_header<'a>( request: &'a super::events::ToolApprovalRequest, model_name: &'a str, ) -> Vec<Line<'a>>`

# Calls

- [time_remaining](../../../../functions/src/tui/events/ToolApprovalRequest/time_remaining.md)

# Called by

- [render_approval](../../../../functions/src/tui/render/render_approval.md)