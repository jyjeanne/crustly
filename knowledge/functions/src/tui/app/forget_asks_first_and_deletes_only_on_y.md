---
type: Rust Function
title: forget_asks_first_and_deletes_only_on_y
resource: src/tui/app.rs#L6306-L6332
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
  - target: functions/src/memory/MemoryStore/find
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/visible_memories
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/handle_key_event
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/key
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn forget_asks_first_and_deletes_only_on_y()`

# Calls

- [memory_screen_app](../../../../functions/src/tui/app/memory_screen_app.md)
- [try_handle_slash_command](../../../../functions/src/tui/app/App/try_handle_slash_command.md)
- [find](../../../../functions/src/memory/MemoryStore/find.md)
- [visible_memories](../../../../functions/src/tui/app/visible_memories.md)
- [handle_key_event](../../../../functions/src/tui/app/App/handle_key_event.md)
- [key](../../../../functions/src/tui/app/key.md)