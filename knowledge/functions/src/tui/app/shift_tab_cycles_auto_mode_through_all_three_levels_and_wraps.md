---
type: Rust Function
title: shift_tab_cycles_auto_mode_through_all_three_levels_and_wraps
resource: src/tui/app.rs#L4840-L4852
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
  - target: functions/src/tui/app/key
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/auto_mode
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn shift_tab_cycles_auto_mode_through_all_three_levels_and_wraps()`

# Calls

- [handle_key_event](../../../../functions/src/tui/app/App/handle_key_event.md)
- [key](../../../../functions/src/tui/app/key.md)
- [auto_mode](../../../../functions/src/tui/app/App/auto_mode.md)