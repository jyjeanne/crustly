---
type: Rust Method
title: copy_last_response_to_clipboard
resource: src/tui/app.rs#L662-L681
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/app/App/last_assistant_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/markdown/last_code_block
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/clipboard
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/notify
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/app/plural
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/app/App/handle_chat_key
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn copy_last_response_to_clipboard(&mut self)`

# Calls

- [last_assistant_message](../../../../../functions/src/tui/app/App/last_assistant_message.md)
- [last_code_block](../../../../../functions/src/tui/markdown/last_code_block.md)
- [clipboard](../../../../../functions/src/tui/app/App/clipboard.md)
- [notify](../../../../../functions/src/tui/app/App/notify.md)
- [plural](../../../../../functions/src/tui/app/plural.md)

# Called by

- [handle_chat_key](../../../../../functions/src/tui/app/App/handle_chat_key.md)