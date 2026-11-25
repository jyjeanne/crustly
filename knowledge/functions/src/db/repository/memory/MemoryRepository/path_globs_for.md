---
type: Rust Method
title: path_globs_for
resource: src/db/repository/memory.rs#L318-L339
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/memory/MemoryStore/try_recall
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/memories_for_path
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn path_globs_for( &self, project_id: &ProjectId, now: DateTime<Utc>, ) -> Result<Vec<(Uuid, String)>>`

# Called by

- [try_recall](../../../../../../functions/src/memory/MemoryStore/try_recall.md)
- [memories_for_path](../../../../../../functions/src/memory/MemoryStore/memories_for_path.md)