---
type: Rust Method
title: list_for_project
resource: src/db/repository/memory.rs#L444-L463
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/memory/consolidate/MemoryStore/consolidate
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/list
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn list_for_project( &self, project_id: &ProjectId, include_inactive: bool, ) -> Result<Vec<Memory>>`

# Called by

- [consolidate](../../../../../../functions/src/memory/consolidate/MemoryStore/consolidate.md)
- [list](../../../../../../functions/src/memory/MemoryStore/list.md)