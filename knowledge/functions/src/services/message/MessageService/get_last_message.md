---
type: Rust Method
title: get_last_message
resource: src/services/message.rs#L173-L176
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/services/message/MessageService/list_messages_for_session
---

# Signature

`pub async fn get_last_message(&self, session_id: Uuid) -> Result<Option<Message>>`

# Calls

- [list_messages_for_session](../../../../../functions/src/services/message/MessageService/list_messages_for_session.md)