---
type: Rust Function
title: render_model_info
resource: src/tui/render.rs#L1557-L1666
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/tui/app/App/last_assistant_message
  - functions/src/plan/PlanTask/block
  called_by:
  - functions/src/tui/render/render
---

# Signature

`fn render_model_info(f: &mut Frame, app: &App, area: Rect)`

# Calls

- [last_assistant_message](../../../../functions/src/tui/app/App/last_assistant_message.md)
- [block](../../../../functions/src/plan/PlanTask/block.md)

# Called by

- [render](../../../../functions/src/tui/render/render.md)