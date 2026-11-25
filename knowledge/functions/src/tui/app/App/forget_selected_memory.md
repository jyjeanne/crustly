---
type: Rust Method
title: forget_selected_memory
resource: src/tui/app.rs#L2049-L2058
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/app/App/selected_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/forget
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/reload_memory_rows
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/app/App/handle_memory_key
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn forget_selected_memory(&mut self)`

# Calls

- [selected_memory](../../../../../functions/src/tui/app/App/selected_memory.md)
- [forget](../../../../../functions/src/memory/MemoryStore/forget.md)
- [reload_memory_rows](../../../../../functions/src/tui/app/App/reload_memory_rows.md)

# Called by

- [handle_memory_key](../../../../../functions/src/tui/app/App/handle_memory_key.md)