---
type: Rust Method
title: update_message_tool_log
resource: src/services/message.rs#L116-L127
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/services/message/MessageService/get_message_required
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/message/MessageRepository/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/ServiceContext/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/message/MessageRepository/update
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn update_message_tool_log(&self, id: Uuid, tool_log: &str) -> Result<()>`

# Calls

- [get_message_required](../../../../../functions/src/services/message/MessageService/get_message_required.md)
- [new](../../../../../functions/src/db/repository/message/MessageRepository/new.md)
- [pool](../../../../../functions/src/services/ServiceContext/pool.md)
- [update](../../../../../functions/src/db/repository/message/MessageRepository/update.md)
- [context](../../../../../functions/src/services/ServiceManager/context.md)

# Called by

- [send_message_with_tools_inner](../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)