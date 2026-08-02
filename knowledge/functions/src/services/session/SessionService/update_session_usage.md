---
type: Rust Method
title: update_session_usage
resource: src/services/session.rs#L103-L121
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/services/session/SessionService/get_session_required
  called_by:
  - functions/src/llm/agent/service/AgentService/send_message
  - functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
  - functions/src/services/session/test_update_session_usage
---

# Signature

`pub async fn update_session_usage(&self, id: Uuid, token_count: i32, cost: f64) -> Result<()>`

# Calls

- [get_session_required](../../../../../functions/src/services/session/SessionService/get_session_required.md)

# Called by

- [send_message](../../../../../functions/src/llm/agent/service/AgentService/send_message.md)
- [send_message_with_tools_inner](../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)
- [test_update_session_usage](../../../../../functions/src/services/session/test_update_session_usage.md)