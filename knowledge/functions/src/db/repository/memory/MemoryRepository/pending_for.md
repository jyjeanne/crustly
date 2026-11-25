---
type: Rust Method
title: pending_for
resource: src/db/repository/memory.rs#L264-L273
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/memory/extraction/MemoryStore/pending
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn pending_for(&self, project_id: &ProjectId) -> Result<Vec<Memory>>`

# Called by

- [pending](../../../../../../functions/src/memory/extraction/MemoryStore/pending.md)