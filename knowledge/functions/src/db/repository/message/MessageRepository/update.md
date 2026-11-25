---
type: Rust Method
title: update
resource: src/db/repository/message.rs#L127-L149
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/db/repository/message/test_message_crud
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
---

# Signature

`pub async fn update(&self, message: &Message) -> Result<()>`

# Calls

- [context](../../../../../../functions/src/services/ServiceManager/context.md)

# Called by

- [test_message_crud](../../../../../../functions/src/db/repository/message/test_message_crud.md)
- [update_message](../../../../../../functions/src/services/message/MessageService/update_message.md)
- [update_message_usage](../../../../../../functions/src/services/message/MessageService/update_message_usage.md)
- [update_message_tool_log](../../../../../../functions/src/services/message/MessageService/update_message_tool_log.md)
- [update_message_metrics](../../../../../../functions/src/services/message/MessageService/update_message_metrics.md)