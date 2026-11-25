---
type: Rust Function
title: memory_line
resource: src/cli/mod.rs#L1101-L1115
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/model/MemoryCategory/as_str
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/model/MemoryStatus/as_str
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/cli/memory_lines_show_a_short_id_scope_category_and_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/cmd_memory
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn memory_line(memory: &crate::memory::model::Memory) -> String`

# Calls

- [as_str](../../../functions/src/memory/model/MemoryCategory/as_str.md)
- [as_str](../../../functions/src/memory/model/MemoryStatus/as_str.md)

# Called by

- [memory_lines_show_a_short_id_scope_category_and_text](../../../functions/src/cli/memory_lines_show_a_short_id_scope_category_and_text.md)
- [cmd_memory](../../../functions/src/cli/cmd_memory.md)