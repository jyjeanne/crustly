---
type: Rust Method
title: events
resource: src/db/repository/memory.rs#L476-L493
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/db/repository/memory/timestamp
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/MemoryStore/events
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn events(&self, id: Uuid) -> Result<Vec<(String, String, DateTime<Utc>)>>`

# Calls

- [timestamp](../../../../../../functions/src/db/repository/memory/timestamp.md)

# Called by

- [events](../../../../../../functions/src/memory/MemoryStore/events.md)