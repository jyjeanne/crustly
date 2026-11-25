---
type: Rust Function
title: test_delete_file
resource: src/services/file.rs#L299-L315
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/services/session/SessionService/create_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/file/FileService/delete_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/file/FileService/get_file
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn test_delete_file()`

# Calls

- [create_session](../../../../functions/src/services/session/SessionService/create_session.md)
- [delete_file](../../../../functions/src/services/file/FileService/delete_file.md)
- [get_file](../../../../functions/src/services/file/FileService/get_file.md)