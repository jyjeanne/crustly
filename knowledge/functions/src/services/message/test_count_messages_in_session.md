---
type: Rust Function
title: test_count_messages_in_session
resource: src/services/message.rs#L460-L481
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/services/message/MessageService/create_message
  - functions/src/services/message/MessageService/count_messages_in_session
---

# Signature

`async fn test_count_messages_in_session()`

# Calls

- [create_message](../../../../functions/src/services/message/MessageService/create_message.md)
- [count_messages_in_session](../../../../functions/src/services/message/MessageService/count_messages_in_session.md)