---
type: Rust Method
title: list_messages_for_session
resource: src/services/message.rs#L76-L81
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
  - functions/src/llm/agent/service/AgentService/prepare_message_context
  - functions/src/services/message/MessageService/get_last_message
  - functions/src/services/message/MessageService/get_messages_by_role
  - functions/src/services/message/MessageService/calculate_total_tokens
  - functions/src/services/message/MessageService/calculate_total_cost
  - functions/src/services/message/test_list_messages_for_session
  - functions/src/services/message/test_delete_messages_for_session
  - functions/src/tui/app/App/load_session
  - functions/src/tui/app/clear_session_is_refused_while_the_current_session_is_processing
  - functions/src/tui/app/clear_session_proceeds_when_only_another_session_is_processing
  - functions/tests/integration_test/test_end_to_end_simple_message
  - functions/tests/integration_test/test_end_to_end_multi_turn_conversation
  - functions/tests/integration_test/test_end_to_end_session_management
  - functions/tests/integration_test/test_end_to_end_token_usage
---

# Signature

`pub async fn list_messages_for_session(&self, session_id: Uuid) -> Result<Vec<Message>>`

# Called by

- [send_message_with_tools_inner](../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)
- [prepare_message_context](../../../../../functions/src/llm/agent/service/AgentService/prepare_message_context.md)
- [get_last_message](../../../../../functions/src/services/message/MessageService/get_last_message.md)
- [get_messages_by_role](../../../../../functions/src/services/message/MessageService/get_messages_by_role.md)
- [calculate_total_tokens](../../../../../functions/src/services/message/MessageService/calculate_total_tokens.md)
- [calculate_total_cost](../../../../../functions/src/services/message/MessageService/calculate_total_cost.md)
- [test_list_messages_for_session](../../../../../functions/src/services/message/test_list_messages_for_session.md)
- [test_delete_messages_for_session](../../../../../functions/src/services/message/test_delete_messages_for_session.md)
- [load_session](../../../../../functions/src/tui/app/App/load_session.md)
- [clear_session_is_refused_while_the_current_session_is_processing](../../../../../functions/src/tui/app/clear_session_is_refused_while_the_current_session_is_processing.md)
- [clear_session_proceeds_when_only_another_session_is_processing](../../../../../functions/src/tui/app/clear_session_proceeds_when_only_another_session_is_processing.md)
- [test_end_to_end_simple_message](../../../../../functions/tests/integration_test/test_end_to_end_simple_message.md)
- [test_end_to_end_multi_turn_conversation](../../../../../functions/tests/integration_test/test_end_to_end_multi_turn_conversation.md)
- [test_end_to_end_session_management](../../../../../functions/tests/integration_test/test_end_to_end_session_management.md)
- [test_end_to_end_token_usage](../../../../../functions/tests/integration_test/test_end_to_end_token_usage.md)