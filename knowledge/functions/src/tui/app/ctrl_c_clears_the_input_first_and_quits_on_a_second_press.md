---
type: Rust Function
title: ctrl_c_clears_the_input_first_and_quits_on_a_second_press
resource: src/tui/app.rs#L4665-L4681
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/app/key_mod
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/handle_event
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/notice
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn ctrl_c_clears_the_input_first_and_quits_on_a_second_press()`

# Calls

- [key_mod](../../../../functions/src/tui/app/key_mod.md)
- [handle_event](../../../../functions/src/tui/app/App/handle_event.md)
- [notice](../../../../functions/src/tui/app/App/notice.md)