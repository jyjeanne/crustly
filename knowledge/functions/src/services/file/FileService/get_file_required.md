---
type: Rust Method
title: get_file_required
resource: src/services/file.rs#L55-L59
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/services/file/FileService/get_file
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/services/file/FileService/update_file_content
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/file/test_update_file_content
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn get_file_required(&self, id: Uuid) -> Result<File>`

# Calls

- [get_file](../../../../../functions/src/services/file/FileService/get_file.md)

# Called by

- [update_file_content](../../../../../functions/src/services/file/FileService/update_file_content.md)
- [test_update_file_content](../../../../../functions/src/services/file/test_update_file_content.md)