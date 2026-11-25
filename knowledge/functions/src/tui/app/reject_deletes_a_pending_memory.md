---
type: Rust Function
title: reject_deletes_a_pending_memory
resource: src/tui/app.rs#L6290-L6303
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
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

`async fn reject_deletes_a_pending_memory()`

# Calls

- [memory_screen_app](../../../../functions/src/tui/app/memory_screen_app.md)
- [try_handle_slash_command](../../../../functions/src/tui/app/App/try_handle_slash_command.md)
- [handle_key_event](../../../../functions/src/tui/app/App/handle_key_event.md)
- [key](../../../../functions/src/tui/app/key.md)
- [identify](../../../../functions/src/memory/project/identify.md)