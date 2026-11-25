---
type: Rust Method
title: pending_replaces
resource: src/db/repository/memory.rs#L254-L261
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/models/PlanTaskStatus/parse
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/extraction/MemoryStore/approve
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn pending_replaces(&self, id: Uuid) -> Result<Option<Uuid>>`

# Calls

- [parse](../../../../../../functions/src/db/models/PlanTaskStatus/parse.md)

# Called by

- [approve](../../../../../../functions/src/memory/extraction/MemoryStore/approve.md)