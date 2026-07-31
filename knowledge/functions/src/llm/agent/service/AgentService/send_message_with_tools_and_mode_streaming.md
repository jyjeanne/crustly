---
type: Rust Method
title: send_message_with_tools_and_mode_streaming
resource: src/llm/agent/service.rs#L801-L817
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
  called_by:
  - functions/src/tui/app/App/send_message
---

# Signature

`pub async fn send_message_with_tools_and_mode_streaming( &self, session_id: Uuid, user_message: String, model: Option<String>, read_only_mode: bool, chunk_tx: mpsc::UnboundedSender<String>, ) -> Result<AgentResponse>`

# Calls

- [send_message_with_tools_inner](../../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)

# Called by

- [send_message](../../../../../../functions/src/tui/app/App/send_message.md)