---
type: Rust Method
title: new
resource: src/db/models.rs#L246-L260
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/db/models/test_message_new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/message/test_message_crud
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/message/test_message_list_by_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn new(session_id: Uuid, role: String, content: String, sequence: i32) -> Self`

# Called by

- [test_message_new](../../../../../functions/src/db/models/test_message_new.md)
- [test_message_crud](../../../../../functions/src/db/repository/message/test_message_crud.md)
- [test_message_list_by_session](../../../../../functions/src/db/repository/message/test_message_list_by_session.md)