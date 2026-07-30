---
type: Rust Function
title: render_model_download
resource: src/tui/render.rs#L1726-L1848
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/tui/render/render_model_download_confirm_delete
  - functions/src/tui/render/render_model_download_deleting
  - functions/src/tui/render/render_model_download_progress
  - functions/src/config/secrets/SecretString/is_empty
  - functions/src/plan/PlanTask/block
  called_by:
  - functions/src/tui/render/render
---

# Signature

`fn render_model_download(f: &mut Frame, app: &App, area: Rect)`

# Calls

- [render_model_download_confirm_delete](../../../../functions/src/tui/render/render_model_download_confirm_delete.md)
- [render_model_download_deleting](../../../../functions/src/tui/render/render_model_download_deleting.md)
- [render_model_download_progress](../../../../functions/src/tui/render/render_model_download_progress.md)
- [is_empty](../../../../functions/src/config/secrets/SecretString/is_empty.md)
- [block](../../../../functions/src/plan/PlanTask/block.md)

# Called by

- [render](../../../../functions/src/tui/render/render.md)