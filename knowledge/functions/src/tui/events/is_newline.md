---
type: Rust Function
title: is_newline
resource: src/tui/events.rs#L533-L538
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/app/App/handle_chat_key
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/events/test_newline_key
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn is_newline(event: &KeyEvent) -> bool`

# Calls

- [contains](../../../../functions/src/memory/vector_index/VectorIndex/contains.md)

# Called by

- [handle_chat_key](../../../../functions/src/tui/app/App/handle_chat_key.md)
- [test_newline_key](../../../../functions/src/tui/events/test_newline_key.md)