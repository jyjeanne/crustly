---
type: Rust Function
title: render_provider_switch
resource: src/tui/render.rs#L1702-L1753
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/is_empty
  - functions/src/plan/PlanTask/block
  called_by:
  - functions/src/tui/render/render
---

# Signature

`fn render_provider_switch(f: &mut Frame, app: &App, area: Rect)`

# Calls

- [is_empty](../../../../functions/src/config/secrets/SecretString/is_empty.md)
- [block](../../../../functions/src/plan/PlanTask/block.md)

# Called by

- [render](../../../../functions/src/tui/render/render.md)