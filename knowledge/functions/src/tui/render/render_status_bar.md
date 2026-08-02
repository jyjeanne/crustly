---
type: Rust Function
title: render_status_bar
resource: src/tui/render.rs#L2324-L2376
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/tui/app/App/auto_mode
  called_by:
  - functions/src/tui/render/render
---

# Signature

`fn render_status_bar(f: &mut Frame, app: &App, area: Rect)`

# Calls

- [auto_mode](../../../../functions/src/tui/app/App/auto_mode.md)

# Called by

- [render](../../../../functions/src/tui/render/render.md)