---
type: Rust Function
title: model_download_backspace_removes_last_char
resource: src/tui/app.rs#L3964-L3974
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
  - target: functions/src/tui/app/App/handle_model_download_key
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/key
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn model_download_backspace_removes_last_char()`

# Calls

- [open_model_download](../../../../functions/src/tui/app/App/open_model_download.md)
- [handle_model_download_key](../../../../functions/src/tui/app/App/handle_model_download_key.md)
- [key](../../../../functions/src/tui/app/key.md)