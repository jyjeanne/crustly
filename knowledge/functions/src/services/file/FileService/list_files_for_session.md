---
type: Rust Method
title: list_files_for_session
resource: src/services/file.rs#L62-L67
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
  - target: functions/src/db/repository/file/FileRepository/find_by_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/services/file/FileService/get_files_with_content
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/file/FileService/get_files_without_content
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/file/test_list_files_for_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/file/test_delete_files_for_session
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn list_files_for_session(&self, session_id: Uuid) -> Result<Vec<File>>`

# Calls

- [new](../../../../../functions/src/db/repository/file/FileRepository/new.md)
- [pool](../../../../../functions/src/services/ServiceContext/pool.md)
- [find_by_session](../../../../../functions/src/db/repository/file/FileRepository/find_by_session.md)

# Called by

- [get_files_with_content](../../../../../functions/src/services/file/FileService/get_files_with_content.md)
- [get_files_without_content](../../../../../functions/src/services/file/FileService/get_files_without_content.md)
- [test_list_files_for_session](../../../../../functions/src/services/file/test_list_files_for_session.md)
- [test_delete_files_for_session](../../../../../functions/src/services/file/test_delete_files_for_session.md)