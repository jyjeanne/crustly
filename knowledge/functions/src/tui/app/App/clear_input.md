---
type: Rust Method
title: clear_input
resource: src/tui/app.rs#L565-L567
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/app/plain_textarea
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/app/App/handle_ctrl_c
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/handle_chat_key
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/chat_input_text_is_not_underlined
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn clear_input(&mut self)`

# Calls

- [plain_textarea](../../../../../functions/src/tui/app/plain_textarea.md)

# Called by

- [handle_ctrl_c](../../../../../functions/src/tui/app/App/handle_ctrl_c.md)
- [handle_chat_key](../../../../../functions/src/tui/app/App/handle_chat_key.md)
- [chat_input_text_is_not_underlined](../../../../../functions/src/tui/app/chat_input_text_is_not_underlined.md)