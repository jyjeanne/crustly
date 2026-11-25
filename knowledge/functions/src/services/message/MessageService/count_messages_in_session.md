---
type: Rust Method
title: count_messages_in_session
resource: src/services/message.rs#L180-L185
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/db/repository/message/MessageRepository/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/ServiceContext/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/message/MessageRepository/count_by_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/services/message/test_count_messages_in_session
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn count_messages_in_session(&self, session_id: Uuid) -> Result<i64>`

# Calls

- [new](../../../../../functions/src/db/repository/message/MessageRepository/new.md)
- [pool](../../../../../functions/src/services/ServiceContext/pool.md)
- [count_by_session](../../../../../functions/src/db/repository/message/MessageRepository/count_by_session.md)

# Called by

- [test_count_messages_in_session](../../../../../functions/src/services/message/test_count_messages_in_session.md)