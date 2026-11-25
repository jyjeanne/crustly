---
type: Rust Method
title: find_by_path
resource: src/db/repository/file.rs#L48-L59
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/services/file/FileService/find_file_by_path
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn find_by_path(&self, session_id: Uuid, path: &Path) -> Result<Option<File>>`

# Calls

- [context](../../../../../../functions/src/services/ServiceManager/context.md)

# Called by

- [find_file_by_path](../../../../../../functions/src/services/file/FileService/find_file_by_path.md)