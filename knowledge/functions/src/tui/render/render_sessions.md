---
type: Rust Function
title: render_sessions
resource: src/tui/render.rs#L528-L578
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/plan/PlanTask/block
  called_by:
  - functions/src/tui/render/render
---

# Signature

`fn render_sessions(f: &mut Frame, app: &App, area: Rect)`

# Calls

- [block](../../../../functions/src/plan/PlanTask/block.md)

# Called by

- [render](../../../../functions/src/tui/render/render.md)