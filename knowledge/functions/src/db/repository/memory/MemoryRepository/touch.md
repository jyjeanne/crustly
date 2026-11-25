---
type: Rust Method
title: touch
resource: src/db/repository/memory.rs#L445-L452
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/memory/MemoryStore/save_with
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn touch(&self, id: Uuid, now: DateTime<Utc>) -> Result<()>`

# Called by

- [save_with](../../../../../../functions/src/memory/MemoryStore/save_with.md)