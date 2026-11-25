---
type: Rust Function
title: model_download_enter_starts_pull_then_esc_aborts_it
resource: src/tui/app.rs#L4003-L4020
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
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

`async fn model_download_enter_starts_pull_then_esc_aborts_it()`

# Calls

- [open_model_download](../../../../functions/src/tui/app/App/open_model_download.md)
- [handle_model_download_key](../../../../functions/src/tui/app/App/handle_model_download_key.md)
- [key](../../../../functions/src/tui/app/key.md)