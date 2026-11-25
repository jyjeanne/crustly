---
type: Rust Method
title: new
resource: src/services/session.rs#L22-L24
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/cmd_run
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/AgentService/prepare_message_context
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/create_test_service
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/test_send_message_with_tool_execution
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/session_for
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/subagents/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/file/create_test_service
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
  - target: functions/src/services/session/create_test_service
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/app/App/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/error_scenarios_test/test_error_api_error
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/error_scenarios_test/test_error_rate_limit
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/error_scenarios_test/test_error_timeout
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/error_scenarios_test/test_error_invalid_response
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/error_scenarios_test/test_error_authentication
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/error_scenarios_test/test_error_empty_message
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/error_scenarios_test/test_error_database_concurrent_access
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/error_scenarios_test/test_error_recovery_after_failure
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
  - target: functions/tests/integration_test/test_end_to_end_cost_tracking
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/integration_test/test_end_to_end_token_usage
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/integration_test/test_end_to_end_system_prompt
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/integration_test/test_database_persistence
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn new(context: ServiceContext) -> Self`

# Called by

- [cmd_run](../../../../../functions/src/cli/cmd_run.md)
- [send_message_with_tools_inner](../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)
- [prepare_message_context](../../../../../functions/src/llm/agent/service/AgentService/prepare_message_context.md)
- [create_test_service](../../../../../functions/src/llm/agent/service/create_test_service.md)
- [test_send_message_with_tool_execution](../../../../../functions/src/llm/agent/service/test_send_message_with_tool_execution.md)
- [session_for](../../../../../functions/src/llm/agent/service/session_for.md)
- [execute](../../../../../functions/src/llm/agent/service/subagents/execute.md)
- [create_test_service](../../../../../functions/src/services/file/create_test_service.md)
- [create_test_service](../../../../../functions/src/services/message/create_test_service.md)
- [create_then_update_survives_a_file_backed_wal_pool](../../../../../functions/src/services/message/create_then_update_survives_a_file_backed_wal_pool.md)
- [new](../../../../../functions/src/services/ServiceManager/new.md)
- [create_test_service](../../../../../functions/src/services/session/create_test_service.md)
- [new](../../../../../functions/src/tui/app/App/new.md)
- [test_error_api_error](../../../../../functions/tests/error_scenarios_test/test_error_api_error.md)
- [test_error_rate_limit](../../../../../functions/tests/error_scenarios_test/test_error_rate_limit.md)
- [test_error_timeout](../../../../../functions/tests/error_scenarios_test/test_error_timeout.md)
- [test_error_invalid_response](../../../../../functions/tests/error_scenarios_test/test_error_invalid_response.md)
- [test_error_authentication](../../../../../functions/tests/error_scenarios_test/test_error_authentication.md)
- [test_error_empty_message](../../../../../functions/tests/error_scenarios_test/test_error_empty_message.md)
- [test_error_database_concurrent_access](../../../../../functions/tests/error_scenarios_test/test_error_database_concurrent_access.md)
- [test_error_recovery_after_failure](../../../../../functions/tests/error_scenarios_test/test_error_recovery_after_failure.md)
- [test_end_to_end_simple_message](../../../../../functions/tests/integration_test/test_end_to_end_simple_message.md)
- [test_end_to_end_multi_turn_conversation](../../../../../functions/tests/integration_test/test_end_to_end_multi_turn_conversation.md)
- [test_end_to_end_session_management](../../../../../functions/tests/integration_test/test_end_to_end_session_management.md)
- [test_end_to_end_cost_tracking](../../../../../functions/tests/integration_test/test_end_to_end_cost_tracking.md)
- [test_end_to_end_token_usage](../../../../../functions/tests/integration_test/test_end_to_end_token_usage.md)
- [test_end_to_end_system_prompt](../../../../../functions/tests/integration_test/test_end_to_end_system_prompt.md)
- [test_database_persistence](../../../../../functions/tests/integration_test/test_database_persistence.md)