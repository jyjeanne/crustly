---
type: Rust Function
title: render_model_download
resource: src/tui/render.rs#L2301-L2423
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/render/render_model_download_confirm_delete
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_model_download_deleting
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_model_download_progress
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/render/render
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn render_model_download(f: &mut Frame, app: &App, area: Rect)`

# Calls

- [render_model_download_confirm_delete](../../../../functions/src/tui/render/render_model_download_confirm_delete.md)
- [render_model_download_deleting](../../../../functions/src/tui/render/render_model_download_deleting.md)
- [render_model_download_progress](../../../../functions/src/tui/render/render_model_download_progress.md)

# Called by

- [render](../../../../functions/src/tui/render/render.md)