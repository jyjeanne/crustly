---
type: Rust Method
title: episodes_for
resource: src/db/repository/memory.rs#L276-L289
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/memory/MemoryStore/recent_work
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn episodes_for(&self, project_id: &ProjectId, limit: i64) -> Result<Vec<Memory>>`

# Called by

- [recent_work](../../../../../../functions/src/memory/MemoryStore/recent_work.md)