---
type: Rust Method
title: paths_of
resource: src/db/repository/memory.rs#L308-L314
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/memory/MemoryStore/add_paths
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn paths_of(&self, id: Uuid) -> Result<Vec<String>>`

# Called by

- [add_paths](../../../../../../functions/src/memory/MemoryStore/add_paths.md)