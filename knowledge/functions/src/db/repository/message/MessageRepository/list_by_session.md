---
type: Rust Method
title: list_by_session
resource: src/db/repository/message.rs#L164-L166
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/repository/message/MessageRepository/find_by_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/db/repository/message/test_message_list_by_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn list_by_session(&self, session_id: Uuid) -> Result<Vec<Message>>`

# Calls

- [find_by_session](../../../../../../functions/src/db/repository/message/MessageRepository/find_by_session.md)

# Called by

- [test_message_list_by_session](../../../../../../functions/src/db/repository/message/test_message_list_by_session.md)