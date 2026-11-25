---
type: Rust Function
title: test_list_sessions
resource: src/services/session.rs#L311-L336
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
  - target: functions/src/services/session/SessionService/list_sessions
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn test_list_sessions()`

# Calls

- [create_session](../../../../functions/src/services/session/SessionService/create_session.md)
- [list_sessions](../../../../functions/src/services/session/SessionService/list_sessions.md)