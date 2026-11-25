---
type: Rust Function
title: test_get_files_with_content
resource: src/services/file.rs#L423-L454
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
  - target: functions/src/services/file/FileService/get_files_with_content
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/file/FileService/get_files_without_content
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn test_get_files_with_content()`

# Calls

- [create_session](../../../../functions/src/services/session/SessionService/create_session.md)
- [get_files_with_content](../../../../functions/src/services/file/FileService/get_files_with_content.md)
- [get_files_without_content](../../../../functions/src/services/file/FileService/get_files_without_content.md)