---
type: Rust Method
title: list_messages_for_session
resource: src/services/message.rs#L77-L82
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/db/repository/message/MessageRepository/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/ServiceContext/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/message/MessageRepository/find_by_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/prepare_message_context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/tool_work_is_remembered_across_requests
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/MessageService/get_last_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/MessageService/get_messages_by_role
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/MessageService/calculate_total_tokens
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/MessageService/calculate_total_cost
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/test_list_messages_for_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/test_delete_messages_for_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/load_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/clear_session_is_refused_while_the_current_session_is_processing
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/clear_session_proceeds_when_only_another_session_is_processing
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/integration_test/test_end_to_end_simple_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/integration_test/test_end_to_end_multi_turn_conversation
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/integration_test/test_end_to_end_session_management
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/integration_test/test_end_to_end_token_usage
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn list_messages_for_session(&self, session_id: Uuid) -> Result<Vec<Message>>`

# Calls

- [new](../../../../../functions/src/db/repository/message/MessageRepository/new.md)
- [pool](../../../../../functions/src/services/ServiceContext/pool.md)
- [find_by_session](../../../../../functions/src/db/repository/message/MessageRepository/find_by_session.md)

# Called by

- [send_message_with_tools_inner](../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)
- [prepare_message_context](../../../../../functions/src/llm/agent/service/AgentService/prepare_message_context.md)
- [tool_work_is_remembered_across_requests](../../../../../functions/src/llm/agent/service/tool_work_is_remembered_across_requests.md)
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