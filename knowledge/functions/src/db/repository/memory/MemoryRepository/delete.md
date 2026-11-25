---
type: Rust Method
title: delete
resource: src/db/repository/memory.rs#L637-L650
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/db/repository/memory/record_event
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/MemoryStore/forget
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn delete(&self, id: Uuid, actor: &str, now: DateTime<Utc>) -> Result<()>`

# Calls

- [record_event](../../../../../../functions/src/db/repository/memory/record_event.md)
- [context](../../../../../../functions/src/services/ServiceManager/context.md)

# Called by

- [forget](../../../../../../functions/src/memory/MemoryStore/forget.md)