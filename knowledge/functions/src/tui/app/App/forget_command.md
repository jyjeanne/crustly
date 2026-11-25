---
type: Rust Method
title: forget_command
resource: src/tui/app.rs#L1763-L1807
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/events/EventHandler/next
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/push_memory_note
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/find
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/forget
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/app/App/try_handle_slash_command
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn forget_command(&mut self, args: &str)`

# Calls

- [next](../../../../../functions/src/tui/events/EventHandler/next.md)
- [push_memory_note](../../../../../functions/src/tui/app/App/push_memory_note.md)
- [find](../../../../../functions/src/memory/MemoryStore/find.md)
- [forget](../../../../../functions/src/memory/MemoryStore/forget.md)

# Called by

- [try_handle_slash_command](../../../../../functions/src/tui/app/App/try_handle_slash_command.md)