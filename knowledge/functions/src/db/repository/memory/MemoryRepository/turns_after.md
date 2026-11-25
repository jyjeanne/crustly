---
type: Rust Method
title: turns_after
resource: src/db/repository/memory.rs#L201-L224
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

`pub async fn turns_after( &self, session: Uuid, sequence: i64, ) -> Result<Vec<(i64, String, String)>>`

# Called by

- [has_unread_turns](../../../../../../functions/src/memory/extraction/MemoryStore/has_unread_turns.md)
- [extract](../../../../../../functions/src/memory/extraction/MemoryStore/extract.md)