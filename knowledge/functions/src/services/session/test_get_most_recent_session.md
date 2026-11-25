---
type: Rust Function
title: test_get_most_recent_session
resource: src/services/session.rs#L339-L356
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
  - target: functions/src/services/session/SessionService/get_most_recent_session
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn test_get_most_recent_session()`

# Calls

- [create_session](../../../../functions/src/services/session/SessionService/create_session.md)
- [get_most_recent_session](../../../../functions/src/services/session/SessionService/get_most_recent_session.md)