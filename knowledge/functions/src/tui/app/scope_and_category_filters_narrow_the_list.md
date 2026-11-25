---
type: Rust Function
title: scope_and_category_filters_narrow_the_list
resource: src/tui/app.rs#L6242-L6257
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
  - target: functions/src/tui/app/visible_memories
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/all
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn scope_and_category_filters_narrow_the_list()`

# Calls

- [memory_screen_app](../../../../functions/src/tui/app/memory_screen_app.md)
- [try_handle_slash_command](../../../../functions/src/tui/app/App/try_handle_slash_command.md)
- [handle_key_event](../../../../functions/src/tui/app/App/handle_key_event.md)
- [key](../../../../functions/src/tui/app/key.md)
- [visible_memories](../../../../functions/src/tui/app/visible_memories.md)
- [all](../../../../functions/src/memory/MemoryStore/all.md)