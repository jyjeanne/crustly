---
type: Rust Function
title: ctrl_o_opens_model_info_panel_and_esc_closes_it
resource: src/tui/app.rs#L5151-L5162
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/app/App/handle_key_event
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/key_mod
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/key
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn ctrl_o_opens_model_info_panel_and_esc_closes_it()`

# Calls

- [handle_key_event](../../../../functions/src/tui/app/App/handle_key_event.md)
- [key_mod](../../../../functions/src/tui/app/key_mod.md)
- [key](../../../../functions/src/tui/app/key.md)