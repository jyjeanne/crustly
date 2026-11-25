---
type: Rust Method
title: refresh_model_download_suggestions
resource: src/tui/app.rs#L3363-L3369
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/ollama_download/filter_suggestions
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/app/App/paste_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/handle_event
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/open_model_download
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/handle_model_download_key
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn refresh_model_download_suggestions(&mut self)`

# Calls

- [filter_suggestions](../../../../../functions/src/tui/ollama_download/filter_suggestions.md)

# Called by

- [paste_text](../../../../../functions/src/tui/app/App/paste_text.md)
- [handle_event](../../../../../functions/src/tui/app/App/handle_event.md)
- [open_model_download](../../../../../functions/src/tui/app/App/open_model_download.md)
- [handle_model_download_key](../../../../../functions/src/tui/app/App/handle_model_download_key.md)