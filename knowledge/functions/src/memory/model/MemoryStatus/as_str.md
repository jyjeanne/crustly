---
type: Rust Method
title: as_str
resource: src/memory/model.rs#L105-L112
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/memory_line
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/cli/cmd_memory
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/memory/MemoryRepository/insert
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn as_str(self) -> &'static str`

# Called by

- [memory_line](../../../../../functions/src/cli/memory_line.md)
- [cmd_memory](../../../../../functions/src/cli/cmd_memory.md)
- [insert](../../../../../functions/src/db/repository/memory/MemoryRepository/insert.md)