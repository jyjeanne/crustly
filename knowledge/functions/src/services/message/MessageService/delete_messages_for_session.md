---
type: Rust Method
title: delete_messages_for_session
resource: src/services/message.rs#L154-L162
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/services/message/test_delete_messages_for_session
  - functions/src/tui/app/App/clear_session
---

# Signature

`pub async fn delete_messages_for_session(&self, session_id: Uuid) -> Result<()>`

# Called by

- [test_delete_messages_for_session](../../../../../functions/src/services/message/test_delete_messages_for_session.md)
- [clear_session](../../../../../functions/src/tui/app/App/clear_session.md)