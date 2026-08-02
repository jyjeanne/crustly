---
type: Rust Function
title: render_model_download_confirm_delete
resource: src/tui/render.rs#L1940-L1981
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/plan/PlanTask/block
  called_by:
  - functions/src/tui/render/render_model_download
---

# Signature

`fn render_model_download_confirm_delete(f: &mut Frame, model: &str, area: Rect)`

# Calls

- [block](../../../../functions/src/plan/PlanTask/block.md)

# Called by

- [render_model_download](../../../../functions/src/tui/render/render_model_download.md)