---
type: Rust Method
title: clamp_memory_selection
resource: src/tui/app.rs#L1889-L1892
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/app/visible_memories
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/app/App/reload_memory_rows
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/handle_memory_key
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn clamp_memory_selection(&mut self)`

# Calls

- [visible_memories](../../../../../functions/src/tui/app/visible_memories.md)

# Called by

- [reload_memory_rows](../../../../../functions/src/tui/app/App/reload_memory_rows.md)
- [handle_memory_key](../../../../../functions/src/tui/app/App/handle_memory_key.md)