---
type: Rust Method
title: send_message_with_tools_and_mode
resource: src/llm/agent/service.rs#L1218-L1234
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/service/AgentService/send_message_with_tools
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn send_message_with_tools_and_mode( &self, session_id: Uuid, user_message: String, model: Option<String>, read_only_mode: bool, ) -> Result<AgentResponse>`

# Calls

- [send_message_with_tools_inner](../../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)

# Called by

- [send_message_with_tools](../../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools.md)