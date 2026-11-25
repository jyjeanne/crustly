---
type: Rust Method
title: delete_by_session
resource: src/db/repository/file.rs#L137-L146
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/services/file/FileService/delete_files_for_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn delete_by_session(&self, session_id: Uuid) -> Result<()>`

# Calls

- [context](../../../../../../functions/src/services/ServiceManager/context.md)

# Called by

- [delete_files_for_session](../../../../../../functions/src/services/file/FileService/delete_files_for_session.md)