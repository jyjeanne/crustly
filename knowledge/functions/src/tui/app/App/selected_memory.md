---
type: Rust Method
title: selected_memory
resource: src/tui/app.rs#L2031-L2035
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/app/visible_memories
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/app/App/handle_memory_key
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/approve_selected_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/forget_selected_memory
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn selected_memory(&self) -> Option<crate::memory::model::Memory>`

# Calls

- [visible_memories](../../../../../functions/src/tui/app/visible_memories.md)

# Called by

- [handle_memory_key](../../../../../functions/src/tui/app/App/handle_memory_key.md)
- [approve_selected_memory](../../../../../functions/src/tui/app/App/approve_selected_memory.md)
- [forget_selected_memory](../../../../../functions/src/tui/app/App/forget_selected_memory.md)