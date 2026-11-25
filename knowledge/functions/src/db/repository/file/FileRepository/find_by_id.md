---
type: Rust Method
title: find_by_id
resource: src/db/repository/file.rs#L24-L32
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/db/repository/file/test_file_crud
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/file/FileService/get_file
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn find_by_id(&self, id: Uuid) -> Result<Option<File>>`

# Calls

- [context](../../../../../../functions/src/services/ServiceManager/context.md)

# Called by

- [test_file_crud](../../../../../../functions/src/db/repository/file/test_file_crud.md)
- [get_file](../../../../../../functions/src/services/file/FileService/get_file.md)