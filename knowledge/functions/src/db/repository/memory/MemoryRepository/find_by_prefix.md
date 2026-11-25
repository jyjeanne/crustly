---
type: Rust Method
title: find_by_prefix
resource: src/db/repository/memory.rs#L409-L418
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/memory/MemoryStore/find
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn find_by_prefix(&self, prefix: &str) -> Result<Vec<Memory>>`

# Called by

- [find](../../../../../../functions/src/memory/MemoryStore/find.md)