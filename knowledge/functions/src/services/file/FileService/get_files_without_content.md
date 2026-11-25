---
type: Rust Method
title: get_files_without_content
resource: src/services/file.rs#L165-L168
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/services/file/FileService/list_files_for_session
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/services/file/test_get_files_with_content
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn get_files_without_content(&self, session_id: Uuid) -> Result<Vec<File>>`

# Calls

- [list_files_for_session](../../../../../functions/src/services/file/FileService/list_files_for_session.md)

# Called by

- [test_get_files_with_content](../../../../../functions/src/services/file/test_get_files_with_content.md)