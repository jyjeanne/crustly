---
type: Rust Method
title: delete
resource: src/db/repository/file.rs#L109-L118
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/db/repository/file/test_file_crud
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/file/FileService/delete_file
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn delete(&self, id: Uuid) -> Result<()>`

# Calls

- [context](../../../../../../functions/src/services/ServiceManager/context.md)

# Called by

- [test_file_crud](../../../../../../functions/src/db/repository/file/test_file_crud.md)
- [delete_file](../../../../../../functions/src/services/file/FileService/delete_file.md)