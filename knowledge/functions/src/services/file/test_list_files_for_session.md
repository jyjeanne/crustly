---
type: Rust Function
title: test_list_files_for_session
resource: src/services/file.rs#L231-L252
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
  - target: functions/src/services/file/FileService/list_files_for_session
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn test_list_files_for_session()`

# Calls

- [create_session](../../../../functions/src/services/session/SessionService/create_session.md)
- [list_files_for_session](../../../../functions/src/services/file/FileService/list_files_for_session.md)