---
type: Rust Function
title: test_get_session
resource: src/services/session.rs#L210-L220
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
  - target: functions/src/services/session/SessionService/get_session
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn test_get_session()`

# Calls

- [create_session](../../../../functions/src/services/session/SessionService/create_session.md)
- [get_session](../../../../functions/src/services/session/SessionService/get_session.md)