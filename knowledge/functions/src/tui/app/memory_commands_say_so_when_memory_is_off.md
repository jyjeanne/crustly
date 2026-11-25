---
type: Rust Function
title: memory_commands_say_so_when_memory_is_off
resource: src/tui/app.rs#L6111-L6118
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/app/App/try_handle_slash_command
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/last_note
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn memory_commands_say_so_when_memory_is_off()`

# Calls

- [try_handle_slash_command](../../../../functions/src/tui/app/App/try_handle_slash_command.md)
- [last_note](../../../../functions/src/tui/app/last_note.md)