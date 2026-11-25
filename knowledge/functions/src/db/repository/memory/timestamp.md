---
type: Rust Function
title: timestamp
resource: src/db/repository/memory.rs#L826-L828
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/db/repository/memory/MemoryRepository/events
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/row_to_memory
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn timestamp(secs: i64) -> DateTime<Utc>`

# Called by

- [events](../../../../../functions/src/db/repository/memory/MemoryRepository/events.md)
- [row_to_memory](../../../../../functions/src/db/repository/memory/row_to_memory.md)