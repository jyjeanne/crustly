---
type: Rust Method
title: last_assistant_message
resource: src/tui/app.rs#L351-L353
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/tui/app/App/copy_last_response_to_clipboard
  - functions/src/tui/render/render_model_info
---

# Signature

`pub fn last_assistant_message(&self) -> Option<&DisplayMessage>`

# Called by

- [copy_last_response_to_clipboard](../../../../../functions/src/tui/app/App/copy_last_response_to_clipboard.md)
- [render_model_info](../../../../../functions/src/tui/render/render_model_info.md)