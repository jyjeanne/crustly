---
type: Rust Function
title: delete_key_on_installed_model_asks_for_confirmation
resource: src/tui/app.rs#L4113-L4136
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/app/App/open_model_download
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/handle_event
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/handle_model_download_key
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/key
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn delete_key_on_installed_model_asks_for_confirmation()`

# Calls

- [open_model_download](../../../../functions/src/tui/app/App/open_model_download.md)
- [handle_event](../../../../functions/src/tui/app/App/handle_event.md)
- [handle_model_download_key](../../../../functions/src/tui/app/App/handle_model_download_key.md)
- [key](../../../../functions/src/tui/app/key.md)