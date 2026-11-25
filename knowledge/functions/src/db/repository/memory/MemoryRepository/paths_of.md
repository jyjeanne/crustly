---
type: Rust Method
title: paths_of
resource: src/db/repository/memory.rs#L381-L387
generated:
  by: okf-rs/0.8.0
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