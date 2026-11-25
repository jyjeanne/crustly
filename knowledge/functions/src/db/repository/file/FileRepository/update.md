---
type: Rust Method
title: update
resource: src/db/repository/file.rs#L86-L106
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
  - target: functions/src/services/file/FileService/update_file
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/file/FileService/update_file_content
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn update(&self, file: &File) -> Result<()>`

# Calls

- [context](../../../../../../functions/src/services/ServiceManager/context.md)

# Called by

- [test_file_crud](../../../../../../functions/src/db/repository/file/test_file_crud.md)
- [update_file](../../../../../../functions/src/services/file/FileService/update_file.md)
- [update_file_content](../../../../../../functions/src/services/file/FileService/update_file_content.md)