---
type: Rust Method
title: relink
resource: src/memory/mod.rs#L758-L763
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/repository/memory/MemoryRepository/upsert_project
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/relink_project
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/changed
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/cmd_memory
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn relink(&self, old_id: &ProjectId, project: &ProjectIdentity) -> Result<u64>`

# Calls

- [upsert_project](../../../../functions/src/db/repository/memory/MemoryRepository/upsert_project.md)
- [relink_project](../../../../functions/src/db/repository/memory/MemoryRepository/relink_project.md)
- [changed](../../../../functions/src/memory/MemoryStore/changed.md)

# Called by

- [cmd_memory](../../../../functions/src/cli/cmd_memory.md)