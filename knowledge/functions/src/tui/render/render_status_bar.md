---
type: Rust Function
title: render_status_bar
resource: src/tui/render.rs#L3063-L3127
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/app/App/auto_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/notice
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/processing_elapsed
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/format_elapsed
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/render/render
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn render_status_bar(f: &mut Frame, app: &App, area: Rect)`

# Calls

- [auto_mode](../../../../functions/src/tui/app/App/auto_mode.md)
- [notice](../../../../functions/src/tui/app/App/notice.md)
- [processing_elapsed](../../../../functions/src/tui/app/App/processing_elapsed.md)
- [format_elapsed](../../../../functions/src/tui/render/format_elapsed.md)

# Called by

- [render](../../../../functions/src/tui/render/render.md)