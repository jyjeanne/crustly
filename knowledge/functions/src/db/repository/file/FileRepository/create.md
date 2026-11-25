---
type: Rust Method
title: create
resource: src/db/repository/file.rs#L62-L83
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
  - target: functions/src/db/repository/file/test_file_list_by_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/file/FileService/track_file
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn create(&self, file: &File) -> Result<()>`

# Calls

- [context](../../../../../../functions/src/services/ServiceManager/context.md)

# Called by

- [test_file_crud](../../../../../../functions/src/db/repository/file/test_file_crud.md)
- [test_file_list_by_session](../../../../../../functions/src/db/repository/file/test_file_list_by_session.md)
- [track_file](../../../../../../functions/src/services/file/FileService/track_file.md)