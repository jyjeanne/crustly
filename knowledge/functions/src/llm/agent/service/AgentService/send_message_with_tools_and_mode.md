---
type: Rust Method
title: send_message_with_tools_and_mode
resource: src/llm/agent/service.rs#L788-L797
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
  called_by:
  - functions/src/llm/agent/service/AgentService/send_message_with_tools
---

# Signature

`pub async fn send_message_with_tools_and_mode( &self, session_id: Uuid, user_message: String, model: Option<String>, read_only_mode: bool, ) -> Result<AgentResponse>`

# Calls

- [send_message_with_tools_inner](../../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)

# Called by

- [send_message_with_tools](../../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools.md)