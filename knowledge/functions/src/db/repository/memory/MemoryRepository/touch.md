---
type: Rust Method
title: touch
resource: src/db/repository/memory.rs#L372-L379
generated:
  by: okf-rs/0.7.0
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