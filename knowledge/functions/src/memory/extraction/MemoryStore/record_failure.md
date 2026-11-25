---
type: Rust Method
title: record_failure
resource: src/memory/extraction.rs#L275-L295
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/repository/memory/MemoryRepository/set_extraction_state
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/extraction/MemoryStore/extract
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn record_failure( &self, session: Uuid, last: i64, highest: i64, failed: i64, now: chrono::DateTime<Utc>, ) -> Result<()>`

# Calls

- [set_extraction_state](../../../../../functions/src/db/repository/memory/MemoryRepository/set_extraction_state.md)

# Called by

- [extract](../../../../../functions/src/memory/extraction/MemoryStore/extract.md)