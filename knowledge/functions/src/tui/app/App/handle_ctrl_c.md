---
type: Rust Method
title: handle_ctrl_c
resource: src/tui/app.rs#L759-L778
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/app/App/cancel_request
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/clear_input
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/notify
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/tui/app/App/handle_key_event
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn handle_ctrl_c(&mut self)`

# Calls

- [cancel_request](../../../../../functions/src/tui/app/App/cancel_request.md)
- [clear_input](../../../../../functions/src/tui/app/App/clear_input.md)
- [notify](../../../../../functions/src/tui/app/App/notify.md)

# Called by

- [handle_key_event](../../../../../functions/src/tui/app/App/handle_key_event.md)