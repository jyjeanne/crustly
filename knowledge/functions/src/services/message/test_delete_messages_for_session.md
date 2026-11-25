---
type: Rust Function
title: test_delete_messages_for_session
resource: src/services/message.rs#L446-L472
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
  - target: functions/src/services/message/MessageService/delete_messages_for_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/MessageService/list_messages_for_session
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn test_delete_messages_for_session()`

# Calls

- [create_session](../../../../functions/src/services/session/SessionService/create_session.md)
- [create_message](../../../../functions/src/services/message/MessageService/create_message.md)
- [delete_messages_for_session](../../../../functions/src/services/message/MessageService/delete_messages_for_session.md)
- [list_messages_for_session](../../../../functions/src/services/message/MessageService/list_messages_for_session.md)