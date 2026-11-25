---
type: Rust Method
title: new
resource: src/db/repository/message.rs#L18-L20
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/db/repository/message/test_message_crud
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/message/test_message_list_by_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/message/MessageService/create_message
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/message/MessageService/get_message
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/message/MessageService/list_messages_for_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/message/MessageService/update_message
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/message/MessageService/update_message_usage
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/message/MessageService/update_message_tool_log
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/message/MessageService/update_message_metrics
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/message/MessageService/delete_message
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/message/MessageService/delete_messages_for_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/message/MessageService/count_messages_in_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn new(pool: SqlitePool) -> Self`

# Called by

- [test_message_crud](../../../../../../functions/src/db/repository/message/test_message_crud.md)
- [test_message_list_by_session](../../../../../../functions/src/db/repository/message/test_message_list_by_session.md)
- [create_message](../../../../../../functions/src/services/message/MessageService/create_message.md)
- [get_message](../../../../../../functions/src/services/message/MessageService/get_message.md)
- [list_messages_for_session](../../../../../../functions/src/services/message/MessageService/list_messages_for_session.md)
- [update_message](../../../../../../functions/src/services/message/MessageService/update_message.md)
- [update_message_usage](../../../../../../functions/src/services/message/MessageService/update_message_usage.md)
- [update_message_tool_log](../../../../../../functions/src/services/message/MessageService/update_message_tool_log.md)
- [update_message_metrics](../../../../../../functions/src/services/message/MessageService/update_message_metrics.md)
- [delete_message](../../../../../../functions/src/services/message/MessageService/delete_message.md)
- [delete_messages_for_session](../../../../../../functions/src/services/message/MessageService/delete_messages_for_session.md)
- [count_messages_in_session](../../../../../../functions/src/services/message/MessageService/count_messages_in_session.md)