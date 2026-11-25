---
type: Rust Function
title: typed_char
resource: src/tui/events.rs#L420-L431
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/events/coalesce_input
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/events/pasted_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/events/is_text_press
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn typed_char(key: &KeyEvent) -> Option<char>`

# Calls

- [contains](../../../../functions/src/memory/vector_index/VectorIndex/contains.md)

# Called by

- [coalesce_input](../../../../functions/src/tui/events/coalesce_input.md)
- [pasted_text](../../../../functions/src/tui/events/pasted_text.md)
- [is_text_press](../../../../functions/src/tui/events/is_text_press.md)