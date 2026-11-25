---
type: Rust Method
title: push_memory_note
resource: src/tui/app.rs#L1687-L1701
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/tui/app/App/remember_command
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/forget_command
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/memory_command
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn push_memory_note(&mut self, content: String)`

# Called by

- [remember_command](../../../../../functions/src/tui/app/App/remember_command.md)
- [forget_command](../../../../../functions/src/tui/app/App/forget_command.md)
- [memory_command](../../../../../functions/src/tui/app/App/memory_command.md)