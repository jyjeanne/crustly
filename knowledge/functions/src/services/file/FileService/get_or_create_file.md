---
type: Rust Method
title: get_or_create_file
resource: src/services/file.rs#L143-L156
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/services/file/FileService/find_file_by_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/file/FileService/track_file
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/services/file/test_get_or_create_file
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn get_or_create_file( &self, session_id: Uuid, path: PathBuf, content: Option<String>, ) -> Result<File>`

# Calls

- [find_file_by_path](../../../../../functions/src/services/file/FileService/find_file_by_path.md)
- [track_file](../../../../../functions/src/services/file/FileService/track_file.md)

# Called by

- [test_get_or_create_file](../../../../../functions/src/services/file/test_get_or_create_file.md)