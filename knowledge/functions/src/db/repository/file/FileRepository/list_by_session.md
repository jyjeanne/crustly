---
type: Rust Method
title: list_by_session
resource: src/db/repository/file.rs#L121-L123
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/repository/file/FileRepository/find_by_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/db/repository/file/test_file_list_by_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn list_by_session(&self, session_id: Uuid) -> Result<Vec<File>>`

# Calls

- [find_by_session](../../../../../../functions/src/db/repository/file/FileRepository/find_by_session.md)

# Called by

- [test_file_list_by_session](../../../../../../functions/src/db/repository/file/test_file_list_by_session.md)