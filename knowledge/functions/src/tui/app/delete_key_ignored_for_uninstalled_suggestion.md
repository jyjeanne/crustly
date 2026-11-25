---
type: Rust Function
title: delete_key_ignored_for_uninstalled_suggestion
resource: src/tui/app.rs#L4099-L4110
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

`async fn delete_key_ignored_for_uninstalled_suggestion()`

# Calls

- [open_model_download](../../../../functions/src/tui/app/App/open_model_download.md)
- [handle_model_download_key](../../../../functions/src/tui/app/App/handle_model_download_key.md)
- [key](../../../../functions/src/tui/app/key.md)