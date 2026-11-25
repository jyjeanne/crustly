---
type: Rust Function
title: memories_opens_the_screen_with_pending_memories_first
resource: src/tui/app.rs#L6229-L6239
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
  - target: functions/src/tui/app/visible_memories
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn memories_opens_the_screen_with_pending_memories_first()`

# Calls

- [memory_screen_app](../../../../functions/src/tui/app/memory_screen_app.md)
- [try_handle_slash_command](../../../../functions/src/tui/app/App/try_handle_slash_command.md)
- [visible_memories](../../../../functions/src/tui/app/visible_memories.md)