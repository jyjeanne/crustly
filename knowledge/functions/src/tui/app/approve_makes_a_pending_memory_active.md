---
type: Rust Function
title: approve_makes_a_pending_memory_active
resource: src/tui/app.rs#L6275-L6287
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/app/memory_screen_app
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/try_handle_slash_command
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/handle_key_event
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/key
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/project/identify
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn approve_makes_a_pending_memory_active()`

# Calls

- [memory_screen_app](../../../../functions/src/tui/app/memory_screen_app.md)
- [try_handle_slash_command](../../../../functions/src/tui/app/App/try_handle_slash_command.md)
- [handle_key_event](../../../../functions/src/tui/app/App/handle_key_event.md)
- [key](../../../../functions/src/tui/app/key.md)
- [identify](../../../../functions/src/memory/project/identify.md)