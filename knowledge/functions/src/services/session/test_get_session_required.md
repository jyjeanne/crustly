---
type: Rust Function
title: test_get_session_required
resource: src/services/session.rs#L223-L236
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
  - target: functions/src/services/session/SessionService/get_session_required
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn test_get_session_required()`

# Calls

- [create_session](../../../../functions/src/services/session/SessionService/create_session.md)
- [get_session_required](../../../../functions/src/services/session/SessionService/get_session_required.md)