---
type: Rust Function
title: render_plan
resource: src/tui/render.rs#L1161-L1183
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/tui/render/render_plan_document
  - functions/src/plan/PlanTask/block
  - functions/src/tui/render/render_plan_empty_state
  called_by:
  - functions/src/tui/render/render
---

# Signature

`fn render_plan(f: &mut Frame, app: &App, area: Rect)`

# Calls

- [render_plan_document](../../../../functions/src/tui/render/render_plan_document.md)
- [block](../../../../functions/src/plan/PlanTask/block.md)
- [render_plan_empty_state](../../../../functions/src/tui/render/render_plan_empty_state.md)

# Called by

- [render](../../../../functions/src/tui/render/render.md)