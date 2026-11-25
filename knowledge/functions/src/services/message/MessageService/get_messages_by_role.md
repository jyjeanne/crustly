---
type: Rust Method
title: get_messages_by_role
resource: src/services/message.rs#L194-L197
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/services/message/MessageService/list_messages_for_session
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/services/message/test_get_messages_by_role
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn get_messages_by_role(&self, session_id: Uuid, role: &str) -> Result<Vec<Message>>`

# Calls

- [list_messages_for_session](../../../../../functions/src/services/message/MessageService/list_messages_for_session.md)

# Called by

- [test_get_messages_by_role](../../../../../functions/src/services/message/test_get_messages_by_role.md)