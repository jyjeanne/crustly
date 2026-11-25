---
type: Rust Method
title: as_str
resource: src/memory/model.rs#L105-L112
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/cli/memory_line
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
- [insert](../../../../../functions/src/db/repository/memory/MemoryRepository/insert.md)