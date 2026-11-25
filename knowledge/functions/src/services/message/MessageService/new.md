---
type: Rust Method
title: new
resource: src/services/message.rs#L19-L21
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/AgentService/prepare_message_context
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/context_overflow_compacts_and_retries_the_round
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/tool_work_is_remembered_across_requests
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/message/create_test_service
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/message/create_then_update_survives_a_file_backed_wal_pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/ServiceManager/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/app/App/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/integration_test/test_end_to_end_simple_message
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/integration_test/test_end_to_end_multi_turn_conversation
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/integration_test/test_end_to_end_session_management
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/integration_test/test_end_to_end_token_usage
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn new(context: ServiceContext) -> Self`

# Called by

- [send_message_with_tools_inner](../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)
- [prepare_message_context](../../../../../functions/src/llm/agent/service/AgentService/prepare_message_context.md)
- [context_overflow_compacts_and_retries_the_round](../../../../../functions/src/llm/agent/service/context_overflow_compacts_and_retries_the_round.md)
- [tool_work_is_remembered_across_requests](../../../../../functions/src/llm/agent/service/tool_work_is_remembered_across_requests.md)
- [create_test_service](../../../../../functions/src/services/message/create_test_service.md)
- [create_then_update_survives_a_file_backed_wal_pool](../../../../../functions/src/services/message/create_then_update_survives_a_file_backed_wal_pool.md)
- [new](../../../../../functions/src/services/ServiceManager/new.md)
- [new](../../../../../functions/src/tui/app/App/new.md)
- [test_end_to_end_simple_message](../../../../../functions/tests/integration_test/test_end_to_end_simple_message.md)
- [test_end_to_end_multi_turn_conversation](../../../../../functions/tests/integration_test/test_end_to_end_multi_turn_conversation.md)
- [test_end_to_end_session_management](../../../../../functions/tests/integration_test/test_end_to_end_session_management.md)
- [test_end_to_end_token_usage](../../../../../functions/tests/integration_test/test_end_to_end_token_usage.md)