---
type: Rust Function
title: render_model_download_progress
resource: src/tui/render.rs#L1851-L1904
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/plan/PlanTask/block
  called_by:
  - functions/src/tui/render/render_model_download
---

# Signature

`fn render_model_download_progress(f: &mut Frame, app: &App, area: Rect)`

# Calls

- [block](../../../../functions/src/plan/PlanTask/block.md)

# Called by

- [render_model_download](../../../../functions/src/tui/render/render_model_download.md)