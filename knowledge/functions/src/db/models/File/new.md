---
type: Rust Method
title: new
resource: src/db/models.rs#L265-L275
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/db/models/test_file_new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/file/test_file_crud
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/file/test_file_list_by_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn new(session_id: Uuid, path: std::path::PathBuf, content: Option<String>) -> Self`

# Called by

- [test_file_new](../../../../../functions/src/db/models/test_file_new.md)
- [test_file_crud](../../../../../functions/src/db/repository/file/test_file_crud.md)
- [test_file_list_by_session](../../../../../functions/src/db/repository/file/test_file_list_by_session.md)