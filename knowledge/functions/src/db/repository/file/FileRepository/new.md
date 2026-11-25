---
type: Rust Method
title: new
resource: src/db/repository/file.rs#L19-L21
generated:
  by: okf-rs/0.7.0
relationships:
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
  - target: functions/src/services/file/FileService/get_file
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/file/FileService/list_files_for_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/file/FileService/find_file_by_path
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/file/FileService/update_file
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/file/FileService/update_file_content
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/file/FileService/delete_file
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/file/FileService/delete_files_for_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/file/FileService/count_files_in_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn new(pool: SqlitePool) -> Self`

# Called by

- [test_file_crud](../../../../../../functions/src/db/repository/file/test_file_crud.md)
- [test_file_list_by_session](../../../../../../functions/src/db/repository/file/test_file_list_by_session.md)
- [track_file](../../../../../../functions/src/services/file/FileService/track_file.md)
- [get_file](../../../../../../functions/src/services/file/FileService/get_file.md)
- [list_files_for_session](../../../../../../functions/src/services/file/FileService/list_files_for_session.md)
- [find_file_by_path](../../../../../../functions/src/services/file/FileService/find_file_by_path.md)
- [update_file](../../../../../../functions/src/services/file/FileService/update_file.md)
- [update_file_content](../../../../../../functions/src/services/file/FileService/update_file_content.md)
- [delete_file](../../../../../../functions/src/services/file/FileService/delete_file.md)
- [delete_files_for_session](../../../../../../functions/src/services/file/FileService/delete_files_for_session.md)
- [count_files_in_session](../../../../../../functions/src/services/file/FileService/count_files_in_session.md)