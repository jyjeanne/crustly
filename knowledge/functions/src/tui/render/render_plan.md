---
type: Rust Function
title: render_plan
resource: src/tui/render.rs#L1635-L1657
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/render/render_plan_document
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_plan_empty_state
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/render/render
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn render_plan(f: &mut Frame, app: &App, area: Rect)`

# Calls

- [render_plan_document](../../../../functions/src/tui/render/render_plan_document.md)
- [render_plan_empty_state](../../../../functions/src/tui/render/render_plan_empty_state.md)

# Called by

- [render](../../../../functions/src/tui/render/render.md)