---
type: Rust Function
title: chat_ctrl_enter_still_submits_as_legacy_alias
resource: src/tui/app.rs#L5213-L5222
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
  - target: functions/src/tui/app/key_mod
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn chat_ctrl_enter_still_submits_as_legacy_alias()`

# Calls

- [handle_chat_key](../../../../functions/src/tui/app/App/handle_chat_key.md)
- [key](../../../../functions/src/tui/app/key.md)
- [key_mod](../../../../functions/src/tui/app/key_mod.md)