---
type: Rust Method
title: send_message
resource: src/llm/agent/service.rs#L665-L736
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/agent/service/AgentService/prepare_message_context
  - functions/src/llm/agent/service/AgentService/final_text_and_thinking
  - functions/src/services/message/MessageService/create_message
  - functions/src/services/message/MessageService/update_message_usage
  - functions/src/services/message/MessageService/update_message_metrics
  - functions/src/services/session/SessionService/update_session_usage
---

# Signature

`pub async fn send_message( &self, session_id: Uuid, user_message: String, model: Option<String>, ) -> Result<AgentResponse>`

# Calls

- [prepare_message_context](../../../../../../functions/src/llm/agent/service/AgentService/prepare_message_context.md)
- [final_text_and_thinking](../../../../../../functions/src/llm/agent/service/AgentService/final_text_and_thinking.md)
- [create_message](../../../../../../functions/src/services/message/MessageService/create_message.md)
- [update_message_usage](../../../../../../functions/src/services/message/MessageService/update_message_usage.md)
- [update_message_metrics](../../../../../../functions/src/services/message/MessageService/update_message_metrics.md)
- [update_session_usage](../../../../../../functions/src/services/session/SessionService/update_session_usage.md)