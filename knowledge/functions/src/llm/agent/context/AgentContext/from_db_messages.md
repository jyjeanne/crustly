---
type: Rust Method
title: from_db_messages
resource: src/llm/agent/context.rs#L76-L102
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/agent/context/AgentContext/add_message
  called_by:
  - functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
  - functions/src/llm/agent/service/AgentService/prepare_message_context
---

# Signature

`pub fn from_db_messages( session_id: Uuid, db_messages: Vec<DbMessage>, max_tokens: usize, ) -> Self`

# Calls

- [add_message](../../../../../../functions/src/llm/agent/context/AgentContext/add_message.md)

# Called by

- [send_message_with_tools_inner](../../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)
- [prepare_message_context](../../../../../../functions/src/llm/agent/service/AgentService/prepare_message_context.md)