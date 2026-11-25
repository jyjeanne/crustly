---
type: Rust Function
title: test_get_messages_by_role
resource: src/services/message.rs#L521-L556
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
  - target: functions/src/services/message/MessageService/get_messages_by_role
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn test_get_messages_by_role()`

# Calls

- [create_session](../../../../functions/src/services/session/SessionService/create_session.md)
- [create_message](../../../../functions/src/services/message/MessageService/create_message.md)
- [get_messages_by_role](../../../../functions/src/services/message/MessageService/get_messages_by_role.md)