---
type: Rust Function
title: timestamp
resource: src/db/repository/memory.rs#L899-L901
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/db/repository/memory/MemoryRepository/anchors_of
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/anchors_for
    resolved_by: tree-sitter
    confidence: exact
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

- [anchors_of](../../../../../functions/src/db/repository/memory/MemoryRepository/anchors_of.md)
- [anchors_for](../../../../../functions/src/db/repository/memory/MemoryRepository/anchors_for.md)
- [events](../../../../../functions/src/db/repository/memory/MemoryRepository/events.md)
- [row_to_memory](../../../../../functions/src/db/repository/memory/row_to_memory.md)