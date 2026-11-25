---
type: Rust Function
title: ctrl_g_does_not_re_trigger_hardware_detection_on_a_second_open
resource: src/tui/app.rs#L5355-L5372
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/app/App/handle_key_event
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/key_mod
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn ctrl_g_does_not_re_trigger_hardware_detection_on_a_second_open()`

# Calls

- [handle_key_event](../../../../functions/src/tui/app/App/handle_key_event.md)
- [key_mod](../../../../functions/src/tui/app/key_mod.md)