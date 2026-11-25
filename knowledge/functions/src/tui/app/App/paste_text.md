---
type: Rust Method
title: paste_text
resource: src/tui/app.rs#L706-L733
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/app/App/notify
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/app/App/refresh_model_download_suggestions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/plural
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/app/App/paste_from_clipboard
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/handle_event
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn paste_text(&mut self, text: &str)`

# Calls

- [notify](../../../../../functions/src/tui/app/App/notify.md)
- [refresh_model_download_suggestions](../../../../../functions/src/tui/app/App/refresh_model_download_suggestions.md)
- [plural](../../../../../functions/src/tui/app/plural.md)

# Called by

- [paste_from_clipboard](../../../../../functions/src/tui/app/App/paste_from_clipboard.md)
- [handle_event](../../../../../functions/src/tui/app/App/handle_event.md)