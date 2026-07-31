---
type: Rust Function
title: clear_session_proceeds_when_only_another_session_is_processing
resource: src/tui/app.rs#L3986-L4019
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/tui/app/App/create_new_session
  - functions/src/services/message/MessageService/create_message
  - functions/src/tui/app/App/clear_session
  - functions/src/services/message/MessageService/list_messages_for_session
---

# Signature

`async fn clear_session_proceeds_when_only_another_session_is_processing()`

# Calls

- [create_new_session](../../../../functions/src/tui/app/App/create_new_session.md)
- [create_message](../../../../functions/src/services/message/MessageService/create_message.md)
- [clear_session](../../../../functions/src/tui/app/App/clear_session.md)
- [list_messages_for_session](../../../../functions/src/services/message/MessageService/list_messages_for_session.md)