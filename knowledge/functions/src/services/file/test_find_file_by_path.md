---
type: Rust Function
title: test_find_file_by_path
resource: src/services/file.rs#L255-L274
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
  - target: functions/src/services/file/FileService/find_file_by_path
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn test_find_file_by_path()`

# Calls

- [create_session](../../../../functions/src/services/session/SessionService/create_session.md)
- [find_file_by_path](../../../../functions/src/services/file/FileService/find_file_by_path.md)