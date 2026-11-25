---
type: Rust Method
title: approve
resource: src/db/repository/memory.rs#L227-L240
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/repository/memory/record_event
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/extraction/MemoryStore/approve
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn approve(&self, id: Uuid, now: DateTime<Utc>) -> Result<()>`

# Calls

- [record_event](../../../../../../functions/src/db/repository/memory/record_event.md)

# Called by

- [approve](../../../../../../functions/src/memory/extraction/MemoryStore/approve.md)