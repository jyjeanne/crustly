---
type: Rust Function
title: ctrl_w_opens_provider_switch_dialog_in_loading_state
resource: src/tui/app.rs#L5225-L5236
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
---

# Signature

`async fn ctrl_w_opens_provider_switch_dialog_in_loading_state()`

# Calls

- [handle_key_event](../../../../functions/src/tui/app/App/handle_key_event.md)
- [key_mod](../../../../functions/src/tui/app/key_mod.md)