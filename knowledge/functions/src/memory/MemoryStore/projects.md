---
type: Rust Method
title: projects
resource: src/memory/mod.rs#L694-L696
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/repository/memory/MemoryRepository/projects
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/cli/cmd_memory
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/transfer/export
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn projects(&self) -> Result<Vec<(ProjectId, String, String, i64)>>`

# Calls

- [projects](../../../../functions/src/db/repository/memory/MemoryRepository/projects.md)

# Called by

- [cmd_memory](../../../../functions/src/cli/cmd_memory.md)
- [export](../../../../functions/src/memory/transfer/export.md)