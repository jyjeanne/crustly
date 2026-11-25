---
type: Rust Function
title: test_delete_session
resource: src/services/session.rs#L297-L308
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
  - target: functions/src/services/session/SessionService/delete_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/session/SessionService/get_session
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn test_delete_session()`

# Calls

- [create_session](../../../../functions/src/services/session/SessionService/create_session.md)
- [delete_session](../../../../functions/src/services/session/SessionService/delete_session.md)
- [get_session](../../../../functions/src/services/session/SessionService/get_session.md)