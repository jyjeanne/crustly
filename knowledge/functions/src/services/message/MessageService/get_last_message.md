---
type: Rust Method
title: get_last_message
resource: src/services/message.rs#L188-L191
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/services/message/MessageService/list_messages_for_session
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn get_last_message(&self, session_id: Uuid) -> Result<Option<Message>>`

# Calls

- [list_messages_for_session](../../../../../functions/src/services/message/MessageService/list_messages_for_session.md)