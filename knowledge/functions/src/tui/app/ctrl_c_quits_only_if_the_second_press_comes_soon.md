---
type: Rust Function
title: ctrl_c_quits_only_if_the_second_press_comes_soon
resource: src/tui/app.rs#L4684-L4697
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
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

`async fn ctrl_c_quits_only_if_the_second_press_comes_soon()`

# Calls

- [key_mod](../../../../functions/src/tui/app/key_mod.md)
- [handle_event](../../../../functions/src/tui/app/App/handle_event.md)
- [notice](../../../../functions/src/tui/app/App/notice.md)