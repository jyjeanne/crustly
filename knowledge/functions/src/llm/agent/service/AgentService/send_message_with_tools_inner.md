---
type: Rust Method
title: send_message_with_tools_inner
resource: src/llm/agent/service.rs#L820-L1573
visibility: private
generated:
  by: okf-rs/0.2.0
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
  - functions/src/tui/prompt_analyzer/PromptAnalyzer/classify_tier
  - functions/src/llm/tools/trait/ToolExecutionContext/with_auto_approve
  - functions/src/llm/tools/trait/ToolExecutionContext/with_read_only_mode
  - functions/src/llm/tools/trait/ToolExecutionContext/with_file_read_cache
  - functions/src/llm/tools/trait/ToolExecutionContext/with_sub_agent_launcher
  - functions/src/llm/provider/types/LLMRequest/with_max_tokens
  - functions/src/llm/provider/types/LLMRequest/with_system
  - functions/src/llm/tools/registry/ToolRegistry/get_tool_definitions
  - functions/src/llm/provider/types/LLMRequest/with_tools
  - functions/src/llm/agent/service/AgentService/call_provider_streaming
  - functions/src/config/secrets/SecretString/is_empty
  - functions/src/llm/agent/service/tool_call_signature
  - functions/src/config/secrets/SecretString/len
  - functions/src/llm/agent/service/has_mutating_capability
  - functions/src/llm/agent/service/is_parallelizable
  - functions/src/llm/tools/cache/CacheKey/from_tool
  - functions/src/llm/tools/cache/ToolResultCache/insert_for_tool
  - functions/src/llm/agent/service/plan_completion_rejection
  - functions/src/llm/tools/registry/ToolRegistry/is_trusted
  - functions/src/llm/tools/cache/ToolResultCache/invalidate_matching
  - functions/src/llm/agent/context/AgentContext/should_compact
  - functions/src/llm/agent/compaction/compact
  - functions/src/llm/agent/service/AgentService/final_text_and_thinking
  - functions/src/services/message/MessageService/update_message_usage
  - functions/src/services/message/MessageService/update_message_metrics
  - functions/src/services/session/SessionService/update_session_usage
  called_by:
  - functions/src/llm/agent/service/AgentService/send_message_with_tools_and_mode
  - functions/src/llm/agent/service/AgentService/send_message_with_tools_and_mode_streaming
---

# Signature

`async fn send_message_with_tools_inner( &self, session_id: Uuid, user_message: String, model: Option<String>, read_only_mode: bool, chunk_tx: Option<mpsc::UnboundedSender<String>>, ) -> Result<AgentResponse>`

# Calls

- [get_session](../../../../../../functions/src/services/session/SessionService/get_session.md)
- [list_messages_for_session](../../../../../../functions/src/services/message/MessageService/list_messages_for_session.md)
- [from_db_messages](../../../../../../functions/src/llm/agent/context/AgentContext/from_db_messages.md)
- [system_prompt_with_env](../../../../../../functions/src/llm/agent/service/AgentService/system_prompt_with_env.md)
- [augment_message_with_pdf](../../../../../../functions/src/llm/pdf_context/augment_message_with_pdf.md)
- [user](../../../../../../functions/src/llm/provider/types/Message/user.md)
- [add_message](../../../../../../functions/src/llm/agent/context/AgentContext/add_message.md)
- [create_message](../../../../../../functions/src/services/message/MessageService/create_message.md)
- [classify_tier](../../../../../../functions/src/tui/prompt_analyzer/PromptAnalyzer/classify_tier.md)
- [with_auto_approve](../../../../../../functions/src/llm/tools/trait/ToolExecutionContext/with_auto_approve.md)
- [with_read_only_mode](../../../../../../functions/src/llm/tools/trait/ToolExecutionContext/with_read_only_mode.md)
- [with_file_read_cache](../../../../../../functions/src/llm/tools/trait/ToolExecutionContext/with_file_read_cache.md)
- [with_sub_agent_launcher](../../../../../../functions/src/llm/tools/trait/ToolExecutionContext/with_sub_agent_launcher.md)
- [with_max_tokens](../../../../../../functions/src/llm/provider/types/LLMRequest/with_max_tokens.md)
- [with_system](../../../../../../functions/src/llm/provider/types/LLMRequest/with_system.md)
- [get_tool_definitions](../../../../../../functions/src/llm/tools/registry/ToolRegistry/get_tool_definitions.md)
- [with_tools](../../../../../../functions/src/llm/provider/types/LLMRequest/with_tools.md)
- [call_provider_streaming](../../../../../../functions/src/llm/agent/service/AgentService/call_provider_streaming.md)
- [is_empty](../../../../../../functions/src/config/secrets/SecretString/is_empty.md)
- [tool_call_signature](../../../../../../functions/src/llm/agent/service/tool_call_signature.md)
- [len](../../../../../../functions/src/config/secrets/SecretString/len.md)
- [has_mutating_capability](../../../../../../functions/src/llm/agent/service/has_mutating_capability.md)
- [is_parallelizable](../../../../../../functions/src/llm/agent/service/is_parallelizable.md)
- [from_tool](../../../../../../functions/src/llm/tools/cache/CacheKey/from_tool.md)
- [insert_for_tool](../../../../../../functions/src/llm/tools/cache/ToolResultCache/insert_for_tool.md)
- [plan_completion_rejection](../../../../../../functions/src/llm/agent/service/plan_completion_rejection.md)
- [is_trusted](../../../../../../functions/src/llm/tools/registry/ToolRegistry/is_trusted.md)
- [invalidate_matching](../../../../../../functions/src/llm/tools/cache/ToolResultCache/invalidate_matching.md)
- [should_compact](../../../../../../functions/src/llm/agent/context/AgentContext/should_compact.md)
- [compact](../../../../../../functions/src/llm/agent/compaction/compact.md)
- [final_text_and_thinking](../../../../../../functions/src/llm/agent/service/AgentService/final_text_and_thinking.md)
- [update_message_usage](../../../../../../functions/src/services/message/MessageService/update_message_usage.md)
- [update_message_metrics](../../../../../../functions/src/services/message/MessageService/update_message_metrics.md)
- [update_session_usage](../../../../../../functions/src/services/session/SessionService/update_session_usage.md)

# Called by

- [send_message_with_tools_and_mode](../../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_and_mode.md)
- [send_message_with_tools_and_mode_streaming](../../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_and_mode_streaming.md)