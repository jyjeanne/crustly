---
type: Rust Function
title: render_approval
resource: src/tui/render.rs#L1901-L1938
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/render/approval_dialog_area
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_approval_header
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_approval_capabilities
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_approval_preview
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_approval_input_detailed
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_approval_input_summary
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_approval_actions
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/render/render
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn render_approval(f: &mut Frame, app: &App, area: Rect)`

# Calls

- [approval_dialog_area](../../../../functions/src/tui/render/approval_dialog_area.md)
- [render_approval_header](../../../../functions/src/tui/render/render_approval_header.md)
- [render_approval_capabilities](../../../../functions/src/tui/render/render_approval_capabilities.md)
- [render_approval_preview](../../../../functions/src/tui/render/render_approval_preview.md)
- [render_approval_input_detailed](../../../../functions/src/tui/render/render_approval_input_detailed.md)
- [render_approval_input_summary](../../../../functions/src/tui/render/render_approval_input_summary.md)
- [render_approval_actions](../../../../functions/src/tui/render/render_approval_actions.md)

# Called by

- [render](../../../../functions/src/tui/render/render.md)