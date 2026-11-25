---
type: Rust Method
title: set_pending_replaces
resource: src/db/repository/memory.rs#L243-L251
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/MemoryStore/save_with
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn set_pending_replaces(&self, id: Uuid, old: Uuid) -> Result<()>`

# Calls

- [context](../../../../../../functions/src/services/ServiceManager/context.md)

# Called by

- [save_with](../../../../../../functions/src/memory/MemoryStore/save_with.md)