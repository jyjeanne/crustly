---
type: Rust Method
title: input_is_blank
resource: src/tui/app.rs#L557-L562
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/all
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/app/App/handle_chat_key
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn input_is_blank(&self) -> bool`

# Calls

- [all](../../../../../functions/src/memory/MemoryStore/all.md)

# Called by

- [handle_chat_key](../../../../../functions/src/tui/app/App/handle_chat_key.md)