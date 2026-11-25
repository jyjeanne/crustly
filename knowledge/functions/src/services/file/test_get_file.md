---
type: Rust Function
title: test_get_file
resource: src/services/file.rs#L213-L228
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
  - target: functions/src/services/file/FileService/get_file
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn test_get_file()`

# Calls

- [create_session](../../../../functions/src/services/session/SessionService/create_session.md)
- [get_file](../../../../functions/src/services/file/FileService/get_file.md)