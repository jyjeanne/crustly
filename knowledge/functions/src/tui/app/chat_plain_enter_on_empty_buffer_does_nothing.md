---
type: Rust Function
title: chat_plain_enter_on_empty_buffer_does_nothing
resource: src/tui/app.rs#L5141-L5148
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/app/App/handle_chat_key
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/key
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn chat_plain_enter_on_empty_buffer_does_nothing()`

# Calls

- [handle_chat_key](../../../../functions/src/tui/app/App/handle_chat_key.md)
- [key](../../../../functions/src/tui/app/key.md)