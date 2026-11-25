---
type: Rust Function
title: test_archive_unarchive_session
resource: src/services/session.rs#L278-L294
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
  - target: functions/src/services/session/SessionService/archive_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/session/SessionService/get_session_required
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/session/SessionService/unarchive_session
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn test_archive_unarchive_session()`

# Calls

- [create_session](../../../../functions/src/services/session/SessionService/create_session.md)
- [archive_session](../../../../functions/src/services/session/SessionService/archive_session.md)
- [get_session_required](../../../../functions/src/services/session/SessionService/get_session_required.md)
- [unarchive_session](../../../../functions/src/services/session/SessionService/unarchive_session.md)