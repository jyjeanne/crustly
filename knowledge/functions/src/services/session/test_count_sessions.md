---
type: Rust Function
title: test_count_sessions
resource: src/services/session.rs#L359-L383
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
  - target: functions/src/services/session/SessionService/archive_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/session/SessionService/count_sessions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/session/SessionService/count_archived_sessions
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn test_count_sessions()`

# Calls

- [create_session](../../../../functions/src/services/session/SessionService/create_session.md)
- [archive_session](../../../../functions/src/services/session/SessionService/archive_session.md)
- [count_sessions](../../../../functions/src/services/session/SessionService/count_sessions.md)
- [count_archived_sessions](../../../../functions/src/services/session/SessionService/count_archived_sessions.md)