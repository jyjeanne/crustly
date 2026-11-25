---
type: Rust Method
title: from_db_messages
resource: src/llm/agent/context.rs#L119-L152
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/context/AgentContext/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/context/AgentContext/add_message
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/prepare_message_context
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn from_db_messages( session_id: Uuid, db_messages: Vec<DbMessage>, max_tokens: usize, ) -> Self`

# Calls

- [new](../../../../../../functions/src/llm/agent/context/AgentContext/new.md)
- [add_message](../../../../../../functions/src/llm/agent/context/AgentContext/add_message.md)

# Called by

- [send_message_with_tools_inner](../../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)
- [prepare_message_context](../../../../../../functions/src/llm/agent/service/AgentService/prepare_message_context.md)