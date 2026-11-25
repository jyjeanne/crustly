---
type: Rust Method
title: get_file
resource: src/services/file.rs#L49-L52
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
  - target: functions/src/db/repository/file/FileRepository/find_by_id
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/services/file/FileService/get_file_required
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/file/test_get_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/file/test_delete_file
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn get_file(&self, id: Uuid) -> Result<Option<File>>`

# Calls

- [new](../../../../../functions/src/db/repository/file/FileRepository/new.md)
- [pool](../../../../../functions/src/services/ServiceContext/pool.md)
- [find_by_id](../../../../../functions/src/db/repository/file/FileRepository/find_by_id.md)
- [context](../../../../../functions/src/services/ServiceManager/context.md)

# Called by

- [get_file_required](../../../../../functions/src/services/file/FileService/get_file_required.md)
- [test_get_file](../../../../../functions/src/services/file/test_get_file.md)
- [test_delete_file](../../../../../functions/src/services/file/test_delete_file.md)