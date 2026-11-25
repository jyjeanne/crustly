---
type: Rust Function
title: test_is_file_tracked
resource: src/services/file.rs#L371-L395
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
  - target: functions/src/services/file/FileService/is_file_tracked
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn test_is_file_tracked()`

# Calls

- [create_session](../../../../functions/src/services/session/SessionService/create_session.md)
- [is_file_tracked](../../../../functions/src/services/file/FileService/is_file_tracked.md)