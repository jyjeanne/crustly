---
type: Rust Function
title: setting_auto_mode_state_shares_the_same_cell_as_a_clone
resource: src/tui/app.rs#L4867-L4882
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/app/App/set_auto_mode_state
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/handle_key_event
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/key
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/auto_mode
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn setting_auto_mode_state_shares_the_same_cell_as_a_clone()`

# Calls

- [set_auto_mode_state](../../../../functions/src/tui/app/App/set_auto_mode_state.md)
- [handle_key_event](../../../../functions/src/tui/app/App/handle_key_event.md)
- [key](../../../../functions/src/tui/app/key.md)
- [auto_mode](../../../../functions/src/tui/app/App/auto_mode.md)