---
type: Rust Function
title: test_get_message
resource: src/services/message.rs#L296-L311
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
  - target: functions/src/services/message/MessageService/create_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/MessageService/get_message
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn test_get_message()`

# Calls

- [create_session](../../../../functions/src/services/session/SessionService/create_session.md)
- [create_message](../../../../functions/src/services/message/MessageService/create_message.md)
- [get_message](../../../../functions/src/services/message/MessageService/get_message.md)