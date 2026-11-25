---
type: Rust Method
title: find_file_by_path
resource: src/services/file.rs#L70-L75
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/db/repository/file/FileRepository/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/ServiceContext/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/file/FileRepository/find_by_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/services/file/FileService/is_file_tracked
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/file/FileService/get_or_create_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/file/test_find_file_by_path
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn find_file_by_path(&self, session_id: Uuid, path: &Path) -> Result<Option<File>>`

# Calls

- [new](../../../../../functions/src/db/repository/file/FileRepository/new.md)
- [pool](../../../../../functions/src/services/ServiceContext/pool.md)
- [find_by_path](../../../../../functions/src/db/repository/file/FileRepository/find_by_path.md)
- [context](../../../../../functions/src/services/ServiceManager/context.md)

# Called by

- [is_file_tracked](../../../../../functions/src/services/file/FileService/is_file_tracked.md)
- [get_or_create_file](../../../../../functions/src/services/file/FileService/get_or_create_file.md)
- [test_find_file_by_path](../../../../../functions/src/services/file/test_find_file_by_path.md)