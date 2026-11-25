---
type: Rust Function
title: recalled_message_can_be_edited_before_resending
resource: src/tui/app.rs#L4308-L4318
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/app/App/push_input_history
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/handle_chat_key
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/key
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/input_text
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn recalled_message_can_be_edited_before_resending()`

# Calls

- [push_input_history](../../../../functions/src/tui/app/App/push_input_history.md)
- [handle_chat_key](../../../../functions/src/tui/app/App/handle_chat_key.md)
- [key](../../../../functions/src/tui/app/key.md)
- [input_text](../../../../functions/src/tui/app/App/input_text.md)