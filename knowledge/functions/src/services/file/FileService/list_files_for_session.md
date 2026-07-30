---
type: Rust Method
title: list_files_for_session
resource: src/services/file.rs#L62-L67
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/services/file/FileService/get_files_with_content
  - functions/src/services/file/FileService/get_files_without_content
  - functions/src/services/file/test_list_files_for_session
  - functions/src/services/file/test_delete_files_for_session
---

# Signature

`pub async fn list_files_for_session(&self, session_id: Uuid) -> Result<Vec<File>>`

# Called by

- [get_files_with_content](../../../../../functions/src/services/file/FileService/get_files_with_content.md)
- [get_files_without_content](../../../../../functions/src/services/file/FileService/get_files_without_content.md)
- [test_list_files_for_session](../../../../../functions/src/services/file/test_list_files_for_session.md)
- [test_delete_files_for_session](../../../../../functions/src/services/file/test_delete_files_for_session.md)