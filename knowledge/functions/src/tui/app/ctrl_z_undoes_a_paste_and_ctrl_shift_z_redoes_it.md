---
type: Rust Function
title: ctrl_z_undoes_a_paste_and_ctrl_shift_z_redoes_it
resource: src/tui/app.rs#L4740-L4762
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
  - target: functions/src/tui/app/App/handle_event
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/key_mod
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/input_text
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn ctrl_z_undoes_a_paste_and_ctrl_shift_z_redoes_it()`

# Calls

- [handle_chat_key](../../../../functions/src/tui/app/App/handle_chat_key.md)
- [key](../../../../functions/src/tui/app/key.md)
- [handle_event](../../../../functions/src/tui/app/App/handle_event.md)
- [key_mod](../../../../functions/src/tui/app/key_mod.md)
- [input_text](../../../../functions/src/tui/app/App/input_text.md)