---
type: Rust Method
title: projects
resource: src/db/repository/memory.rs#L569-L587
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/memory/MemoryStore/projects
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn projects(&self) -> Result<Vec<(ProjectId, String, String, i64)>>`

# Called by

- [projects](../../../../../../functions/src/memory/MemoryStore/projects.md)