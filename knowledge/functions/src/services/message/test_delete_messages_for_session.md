---
type: Rust Function
title: test_delete_messages_for_session
resource: src/services/message.rs#L431-L457
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/services/message/MessageService/create_message
  - functions/src/services/message/MessageService/delete_messages_for_session
  - functions/src/services/message/MessageService/list_messages_for_session
---

# Signature

`async fn test_delete_messages_for_session()`

# Calls

- [create_message](../../../../functions/src/services/message/MessageService/create_message.md)
- [delete_messages_for_session](../../../../functions/src/services/message/MessageService/delete_messages_for_session.md)
- [list_messages_for_session](../../../../functions/src/services/message/MessageService/list_messages_for_session.md)