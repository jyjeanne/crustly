---
type: Rust Method
title: count_messages_in_session
resource: src/services/message.rs#L165-L170
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/services/message/test_count_messages_in_session
---

# Signature

`pub async fn count_messages_in_session(&self, session_id: Uuid) -> Result<i64>`

# Called by

- [test_count_messages_in_session](../../../../../functions/src/services/message/test_count_messages_in_session.md)