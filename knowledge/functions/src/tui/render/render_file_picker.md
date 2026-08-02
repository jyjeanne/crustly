---
type: Rust Function
title: render_file_picker
resource: src/tui/render.rs#L1445-L1562
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/len
  - functions/src/plan/PlanTask/skip
  - functions/src/plan/PlanTask/block
  called_by:
  - functions/src/tui/render/render
---

# Signature

`fn render_file_picker(f: &mut Frame, app: &App, area: Rect)`

# Calls

- [len](../../../../functions/src/config/secrets/SecretString/len.md)
- [skip](../../../../functions/src/plan/PlanTask/skip.md)
- [block](../../../../functions/src/plan/PlanTask/block.md)

# Called by

- [render](../../../../functions/src/tui/render/render.md)