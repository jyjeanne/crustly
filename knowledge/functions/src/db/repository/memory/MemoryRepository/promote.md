---
type: Rust Method
title: promote
resource: src/db/repository/memory.rs#L692-L705
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/record_event
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/MemoryStore/promote
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn promote(&self, id: Uuid, actor: &str, now: DateTime<Utc>) -> Result<()>`

# Calls

- [context](../../../../../../functions/src/services/ServiceManager/context.md)
- [record_event](../../../../../../functions/src/db/repository/memory/record_event.md)

# Called by

- [promote](../../../../../../functions/src/memory/MemoryStore/promote.md)