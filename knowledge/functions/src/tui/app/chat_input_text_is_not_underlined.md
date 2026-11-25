---
type: Rust Function
title: chat_input_text_is_not_underlined
resource: src/tui/app.rs#L4999-L5022
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/app/App/clear_input
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/set_input_text
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn chat_input_text_is_not_underlined()`

# Calls

- [clear_input](../../../../functions/src/tui/app/App/clear_input.md)
- [set_input_text](../../../../functions/src/tui/app/App/set_input_text.md)