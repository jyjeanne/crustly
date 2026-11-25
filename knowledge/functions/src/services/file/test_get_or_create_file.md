---
type: Rust Function
title: test_get_or_create_file
resource: src/services/file.rs#L398-L420
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
  - target: functions/src/services/file/FileService/get_or_create_file
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn test_get_or_create_file()`

# Calls

- [create_session](../../../../functions/src/services/session/SessionService/create_session.md)
- [get_or_create_file](../../../../functions/src/services/file/FileService/get_or_create_file.md)