---
type: Rust Method
title: announce_memory_saves
resource: src/tui/app.rs#L874-L883
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/take_notices
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/notify
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/tui/app/App/handle_event
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/remember_command
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/a_save_during_a_response_is_announced_in_the_chat
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn announce_memory_saves(&mut self)`

# Calls

- [take_notices](../../../../../functions/src/memory/MemoryStore/take_notices.md)
- [notify](../../../../../functions/src/tui/app/App/notify.md)

# Called by

- [handle_event](../../../../../functions/src/tui/app/App/handle_event.md)
- [remember_command](../../../../../functions/src/tui/app/App/remember_command.md)
- [a_save_during_a_response_is_announced_in_the_chat](../../../../../functions/src/tui/app/a_save_during_a_response_is_announced_in_the_chat.md)