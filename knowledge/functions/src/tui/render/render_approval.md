---
type: Rust Function
title: render_approval
resource: src/tui/render.rs#L1407-L1442
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/tui/render/approval_dialog_area
  - functions/src/tui/render/render_approval_header
  - functions/src/tui/render/render_approval_capabilities
  - functions/src/tui/render/render_approval_input_detailed
  - functions/src/tui/render/render_approval_input_summary
  - functions/src/tui/render/render_approval_actions
  - functions/src/plan/PlanTask/block
  called_by:
  - functions/src/tui/render/render
---

# Signature

`fn render_approval(f: &mut Frame, app: &App, area: Rect)`

# Calls

- [approval_dialog_area](../../../../functions/src/tui/render/approval_dialog_area.md)
- [render_approval_header](../../../../functions/src/tui/render/render_approval_header.md)
- [render_approval_capabilities](../../../../functions/src/tui/render/render_approval_capabilities.md)
- [render_approval_input_detailed](../../../../functions/src/tui/render/render_approval_input_detailed.md)
- [render_approval_input_summary](../../../../functions/src/tui/render/render_approval_input_summary.md)
- [render_approval_actions](../../../../functions/src/tui/render/render_approval_actions.md)
- [block](../../../../functions/src/plan/PlanTask/block.md)

# Called by

- [render](../../../../functions/src/tui/render/render.md)