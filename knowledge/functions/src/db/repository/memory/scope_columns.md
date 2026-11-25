---
type: Rust Function
title: scope_columns
resource: src/db/repository/memory.rs#L870-L875
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/db/repository/memory/MemoryRepository/insert
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/find_by_hash
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn scope_columns(scope: &MemoryScope) -> (&'static str, Option<&str>)`

# Called by

- [insert](../../../../../functions/src/db/repository/memory/MemoryRepository/insert.md)
- [find_by_hash](../../../../../functions/src/db/repository/memory/MemoryRepository/find_by_hash.md)