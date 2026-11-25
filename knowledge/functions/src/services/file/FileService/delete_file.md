---
type: Rust Method
title: delete_file
resource: src/services/file.rs#L109-L115
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/repository/file/FileRepository/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/ServiceContext/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/file/FileRepository/delete
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/services/file/test_delete_file
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn delete_file(&self, id: Uuid) -> Result<()>`

# Calls

- [new](../../../../../functions/src/db/repository/file/FileRepository/new.md)
- [pool](../../../../../functions/src/services/ServiceContext/pool.md)
- [delete](../../../../../functions/src/db/repository/file/FileRepository/delete.md)
- [context](../../../../../functions/src/services/ServiceManager/context.md)

# Called by

- [test_delete_file](../../../../../functions/src/services/file/test_delete_file.md)