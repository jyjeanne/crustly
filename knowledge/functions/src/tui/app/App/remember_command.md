---
type: Rust Method
title: remember_command
resource: src/tui/app.rs#L1704-L1759
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/app/App/push_memory_note
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/project/identify
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/register_project
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/announce_memory_saves
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/app/App/try_handle_slash_command
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn remember_command(&mut self, args: &str)`

# Calls

- [push_memory_note](../../../../../functions/src/tui/app/App/push_memory_note.md)
- [identify](../../../../../functions/src/memory/project/identify.md)
- [register_project](../../../../../functions/src/memory/MemoryStore/register_project.md)
- [announce_memory_saves](../../../../../functions/src/tui/app/App/announce_memory_saves.md)

# Called by

- [try_handle_slash_command](../../../../../functions/src/tui/app/App/try_handle_slash_command.md)