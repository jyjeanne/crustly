---
type: Rust Method
title: prepare_message_context
resource: src/llm/agent/service.rs#L1597-L1658
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/services/session/SessionService/get_session
  - functions/src/services/message/MessageService/list_messages_for_session
  - functions/src/llm/agent/context/AgentContext/from_db_messages
  - functions/src/llm/agent/service/AgentService/system_prompt_with_env
  - functions/src/llm/pdf_context/augment_message_with_pdf
  - functions/src/llm/provider/types/Message/user
  - functions/src/llm/agent/context/AgentContext/add_message
  - functions/src/services/message/MessageService/create_message
  - functions/src/llm/provider/types/LLMRequest/with_max_tokens
  - functions/src/llm/provider/types/LLMRequest/with_system
  called_by:
  - functions/src/llm/agent/service/AgentService/send_message
  - functions/src/llm/agent/service/AgentService/send_message_streaming
---

# Signature

`async fn prepare_message_context( &self, session_id: Uuid, user_message: String, model: Option<String>, ) -> Result<(String, LLMRequest, MessageService, SessionService)>`

# Calls

- [get_session](../../../../../../functions/src/services/session/SessionService/get_session.md)
- [list_messages_for_session](../../../../../../functions/src/services/message/MessageService/list_messages_for_session.md)
- [from_db_messages](../../../../../../functions/src/llm/agent/context/AgentContext/from_db_messages.md)
- [system_prompt_with_env](../../../../../../functions/src/llm/agent/service/AgentService/system_prompt_with_env.md)
- [augment_message_with_pdf](../../../../../../functions/src/llm/pdf_context/augment_message_with_pdf.md)
- [user](../../../../../../functions/src/llm/provider/types/Message/user.md)
- [add_message](../../../../../../functions/src/llm/agent/context/AgentContext/add_message.md)
- [create_message](../../../../../../functions/src/services/message/MessageService/create_message.md)
- [with_max_tokens](../../../../../../functions/src/llm/provider/types/LLMRequest/with_max_tokens.md)
- [with_system](../../../../../../functions/src/llm/provider/types/LLMRequest/with_system.md)

# Called by

- [send_message](../../../../../../functions/src/llm/agent/service/AgentService/send_message.md)
- [send_message_streaming](../../../../../../functions/src/llm/agent/service/AgentService/send_message_streaming.md)