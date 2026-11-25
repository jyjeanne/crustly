---
type: Rust Function
title: test_update_session_usage
resource: src/services/session.rs#L256-L275
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
  - target: functions/src/services/session/SessionService/update_session_usage
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/session/SessionService/get_session_required
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn test_update_session_usage()`

# Calls

- [create_session](../../../../functions/src/services/session/SessionService/create_session.md)
- [update_session_usage](../../../../functions/src/services/session/SessionService/update_session_usage.md)
- [get_session_required](../../../../functions/src/services/session/SessionService/get_session_required.md)