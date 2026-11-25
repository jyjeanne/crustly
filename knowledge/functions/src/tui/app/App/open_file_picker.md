---
type: Rust Method
title: open_file_picker
resource: src/tui/app.rs#L3250-L3283
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/app/App/switch_mode
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/app/App/handle_chat_key
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/handle_file_picker_key
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn open_file_picker(&mut self) -> Result<()>`

# Calls

- [switch_mode](../../../../../functions/src/tui/app/App/switch_mode.md)

# Called by

- [handle_chat_key](../../../../../functions/src/tui/app/App/handle_chat_key.md)
- [handle_file_picker_key](../../../../../functions/src/tui/app/App/handle_file_picker_key.md)