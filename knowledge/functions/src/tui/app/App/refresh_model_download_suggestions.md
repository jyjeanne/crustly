---
type: Rust Method
title: refresh_model_download_suggestions
resource: src/tui/app.rs#L2515-L2521
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/tui/ollama_download/filter_suggestions
  called_by:
  - functions/src/tui/app/App/handle_event
  - functions/src/tui/app/App/open_model_download
  - functions/src/tui/app/App/handle_model_download_key
---

# Signature

`fn refresh_model_download_suggestions(&mut self)`

# Calls

- [filter_suggestions](../../../../../functions/src/tui/ollama_download/filter_suggestions.md)

# Called by

- [handle_event](../../../../../functions/src/tui/app/App/handle_event.md)
- [open_model_download](../../../../../functions/src/tui/app/App/open_model_download.md)
- [handle_model_download_key](../../../../../functions/src/tui/app/App/handle_model_download_key.md)