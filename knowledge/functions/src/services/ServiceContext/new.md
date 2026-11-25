---
type: Rust Method
title: new
resource: src/services/mod.rs#L28-L32
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/cli/cmd_chat
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/cli/cmd_run
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/create_test_service
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/test_send_message_with_tool_execution
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/test_context
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
  - target: functions/src/services/test_service_context_creation
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/plan/setup_test_service
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/session/create_test_service
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/app/test_app
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/app/app_with_extraction
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/render/test_app
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/runner/run_loop_exits_immediately_when_should_quit_is_set
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/error_scenarios_test/create_error_agent
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/error_scenarios_test/test_error_empty_message
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/error_scenarios_test/test_error_database_concurrent_access
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/integration_test/create_test_agent
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/integration_test/test_end_to_end_system_prompt
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/integration_test/test_database_persistence
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/plan_mode_integration_test/setup_test_env
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn new(pool: Pool) -> Self`

# Called by

- [cmd_chat](../../../../functions/src/cli/cmd_chat.md)
- [cmd_run](../../../../functions/src/cli/cmd_run.md)
- [create_test_service](../../../../functions/src/llm/agent/service/create_test_service.md)
- [test_send_message_with_tool_execution](../../../../functions/src/llm/agent/service/test_send_message_with_tool_execution.md)
- [test_context](../../../../functions/src/llm/agent/service/test_context.md)
- [create_test_service](../../../../functions/src/services/file/create_test_service.md)
- [create_test_service](../../../../functions/src/services/message/create_test_service.md)
- [create_then_update_survives_a_file_backed_wal_pool](../../../../functions/src/services/message/create_then_update_survives_a_file_backed_wal_pool.md)
- [new](../../../../functions/src/services/ServiceManager/new.md)
- [test_service_context_creation](../../../../functions/src/services/test_service_context_creation.md)
- [setup_test_service](../../../../functions/src/services/plan/setup_test_service.md)
- [create_test_service](../../../../functions/src/services/session/create_test_service.md)
- [test_app](../../../../functions/src/tui/app/test_app.md)
- [app_with_extraction](../../../../functions/src/tui/app/app_with_extraction.md)
- [test_app](../../../../functions/src/tui/render/test_app.md)
- [run_loop_exits_immediately_when_should_quit_is_set](../../../../functions/src/tui/runner/run_loop_exits_immediately_when_should_quit_is_set.md)
- [create_error_agent](../../../../functions/tests/error_scenarios_test/create_error_agent.md)
- [test_error_empty_message](../../../../functions/tests/error_scenarios_test/test_error_empty_message.md)
- [test_error_database_concurrent_access](../../../../functions/tests/error_scenarios_test/test_error_database_concurrent_access.md)
- [create_test_agent](../../../../functions/tests/integration_test/create_test_agent.md)
- [test_end_to_end_system_prompt](../../../../functions/tests/integration_test/test_end_to_end_system_prompt.md)
- [test_database_persistence](../../../../functions/tests/integration_test/test_database_persistence.md)
- [setup_test_env](../../../../functions/tests/plan_mode_integration_test/setup_test_env.md)