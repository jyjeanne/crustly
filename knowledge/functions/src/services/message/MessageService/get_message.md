---
type: Rust Method
title: get_message
resource: src/services/message.rs#L63-L66
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/services/message/MessageService/get_message_required
  - functions/src/services/message/test_get_message
  - functions/src/services/message/test_delete_message
---

# Signature

`pub async fn get_message(&self, id: Uuid) -> Result<Option<Message>>`

# Called by

- [get_message_required](../../../../../functions/src/services/message/MessageService/get_message_required.md)
- [test_get_message](../../../../../functions/src/services/message/test_get_message.md)
- [test_delete_message](../../../../../functions/src/services/message/test_delete_message.md)