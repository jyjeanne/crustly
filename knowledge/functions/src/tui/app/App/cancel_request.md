---
type: Rust Method
title: cancel_request
resource: src/tui/app.rs#L526-L535
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/tui/app/App/handle_ctrl_c
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/handle_chat_key
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn cancel_request(&mut self) -> bool`

# Called by

- [handle_ctrl_c](../../../../../functions/src/tui/app/App/handle_ctrl_c.md)
- [handle_chat_key](../../../../../functions/src/tui/app/App/handle_chat_key.md)