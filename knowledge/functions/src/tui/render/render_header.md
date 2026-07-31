---
type: Rust Function
title: render_header
resource: src/tui/render.rs#L101-L197
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/tui/app/App/total_tokens
  - functions/src/tui/app/App/total_cost
  - functions/src/config/secrets/SecretString/len
  - functions/src/plan/PlanTask/block
  called_by:
  - functions/src/tui/render/render
---

# Signature

`fn render_header(f: &mut Frame, app: &App, area: Rect)`

# Calls

- [total_tokens](../../../../functions/src/tui/app/App/total_tokens.md)
- [total_cost](../../../../functions/src/tui/app/App/total_cost.md)
- [len](../../../../functions/src/config/secrets/SecretString/len.md)
- [block](../../../../functions/src/plan/PlanTask/block.md)

# Called by

- [render](../../../../functions/src/tui/render/render.md)