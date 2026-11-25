---
type: Rust Function
title: test_update_file_content
resource: src/services/file.rs#L277-L296
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/services/session/SessionService/create_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/file/FileService/update_file_content
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/file/FileService/get_file_required
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn test_update_file_content()`

# Calls

- [create_session](../../../../functions/src/services/session/SessionService/create_session.md)
- [update_file_content](../../../../functions/src/services/file/FileService/update_file_content.md)
- [get_file_required](../../../../functions/src/services/file/FileService/get_file_required.md)