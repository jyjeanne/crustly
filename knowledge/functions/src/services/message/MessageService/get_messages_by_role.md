---
type: Rust Method
title: get_messages_by_role
resource: src/services/message.rs#L179-L182
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/services/message/MessageService/list_messages_for_session
  called_by:
  - functions/src/services/message/test_get_messages_by_role
---

# Signature

`pub async fn get_messages_by_role(&self, session_id: Uuid, role: &str) -> Result<Vec<Message>>`

# Calls

- [list_messages_for_session](../../../../../functions/src/services/message/MessageService/list_messages_for_session.md)

# Called by

- [test_get_messages_by_role](../../../../../functions/src/services/message/test_get_messages_by_role.md)