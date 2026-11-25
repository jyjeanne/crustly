---
type: Rust Method
title: add_paths
resource: src/db/repository/memory.rs#L294-L305
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/memory/MemoryStore/add_paths
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn add_paths(&self, id: Uuid, globs: &[String]) -> Result<()>`

# Called by

- [add_paths](../../../../../../functions/src/memory/MemoryStore/add_paths.md)