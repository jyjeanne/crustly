---
type: Rust Method
title: send_message_streaming
resource: src/llm/agent/service.rs#L741-L768
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/agent/service/AgentService/prepare_message_context
  - functions/src/llm/provider/types/LLMRequest/with_streaming
---

# Signature

`pub async fn send_message_streaming( &self, session_id: Uuid, user_message: String, model: Option<String>, ) -> Result<AgentStreamResponse>`

# Calls

- [prepare_message_context](../../../../../../functions/src/llm/agent/service/AgentService/prepare_message_context.md)
- [with_streaming](../../../../../../functions/src/llm/provider/types/LLMRequest/with_streaming.md)