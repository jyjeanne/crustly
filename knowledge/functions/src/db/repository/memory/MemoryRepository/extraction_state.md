---
type: Rust Method
title: extraction_state
resource: src/db/repository/memory.rs#L160-L175
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/memory/extraction/MemoryStore/has_unread_turns
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/extraction/MemoryStore/extract
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn extraction_state(&self, session: Uuid) -> Result<Option<(i64, i64)>>`

# Called by

- [has_unread_turns](../../../../../../functions/src/memory/extraction/MemoryStore/has_unread_turns.md)
- [extract](../../../../../../functions/src/memory/extraction/MemoryStore/extract.md)