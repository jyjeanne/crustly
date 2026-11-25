---
type: Rust Method
title: set_extraction_state
resource: src/db/repository/memory.rs#L177-L197
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/memory/extraction/MemoryStore/extract
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/extraction/MemoryStore/record_failure
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn set_extraction_state( &self, session: Uuid, sequence: i64, failed: i64, now: DateTime<Utc>, ) -> Result<()>`

# Called by

- [extract](../../../../../../functions/src/memory/extraction/MemoryStore/extract.md)
- [record_failure](../../../../../../functions/src/memory/extraction/MemoryStore/record_failure.md)