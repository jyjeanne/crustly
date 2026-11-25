---
type: Rust Method
title: send_message_streaming
resource: src/llm/agent/service.rs#L1171-L1198
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/AgentService/prepare_message_context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/LLMRequest/with_streaming
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn send_message_streaming( &self, session_id: Uuid, user_message: String, model: Option<String>, ) -> Result<AgentStreamResponse>`

# Calls

- [prepare_message_context](../../../../../../functions/src/llm/agent/service/AgentService/prepare_message_context.md)
- [with_streaming](../../../../../../functions/src/llm/provider/types/LLMRequest/with_streaming.md)