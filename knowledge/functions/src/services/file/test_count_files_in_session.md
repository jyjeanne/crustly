---
type: Rust Function
title: test_count_files_in_session
resource: src/services/file.rs#L347-L368
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
  - target: functions/src/services/file/FileService/count_files_in_session
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn test_count_files_in_session()`

# Calls

- [create_session](../../../../functions/src/services/session/SessionService/create_session.md)
- [count_files_in_session](../../../../functions/src/services/file/FileService/count_files_in_session.md)