---
type: Rust Function
title: test_count_messages_in_session
resource: src/services/message.rs#L475-L496
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
  - target: functions/src/services/message/MessageService/create_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/MessageService/count_messages_in_session
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn test_count_messages_in_session()`

# Calls

- [create_session](../../../../functions/src/services/session/SessionService/create_session.md)
- [create_message](../../../../functions/src/services/message/MessageService/create_message.md)
- [count_messages_in_session](../../../../functions/src/services/message/MessageService/count_messages_in_session.md)