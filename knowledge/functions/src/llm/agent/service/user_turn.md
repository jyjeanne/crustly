---
type: Rust Function
title: user_turn
resource: src/llm/agent/service.rs#L548-L561
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/types/Message/user
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

`fn user_turn(block: Option<String>, text: &str) -> Message`

# Calls

- [user](../../../../../functions/src/llm/provider/types/Message/user.md)

# Called by

- [send_message_with_tools_inner](../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)
- [prepare_message_context](../../../../../functions/src/llm/agent/service/AgentService/prepare_message_context.md)