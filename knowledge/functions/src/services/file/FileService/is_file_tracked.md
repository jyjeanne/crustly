---
type: Rust Method
title: is_file_tracked
resource: src/services/file.rs#L137-L140
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/services/file/FileService/find_file_by_path
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/services/file/test_is_file_tracked
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn is_file_tracked(&self, session_id: Uuid, path: &Path) -> Result<bool>`

# Calls

- [find_file_by_path](../../../../../functions/src/services/file/FileService/find_file_by_path.md)

# Called by

- [test_is_file_tracked](../../../../../functions/src/services/file/test_is_file_tracked.md)