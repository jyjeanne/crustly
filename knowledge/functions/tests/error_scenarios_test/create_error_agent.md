---
type: Rust Function
title: create_error_agent
resource: tests/error_scenarios_test.rs#L94-L110
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/tests/error_scenarios_test/ErrorMockProvider/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/ServiceContext/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/Database/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/registry/ToolRegistry/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/registry/ToolRegistry/register
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/AgentService/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/AgentService/with_tool_registry
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/tests/error_scenarios_test/test_error_api_error
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/error_scenarios_test/test_error_rate_limit
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/error_scenarios_test/test_error_timeout
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/error_scenarios_test/test_error_invalid_response
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/error_scenarios_test/test_error_authentication
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/error_scenarios_test/test_error_session_not_found
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/error_scenarios_test/test_error_recovery_after_failure
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn create_error_agent( db: &Database, error_type: ErrorType, ) -> Result<(AgentService, ServiceContext)>`

# Calls

- [new](../../../functions/tests/error_scenarios_test/ErrorMockProvider/new.md)
- [new](../../../functions/src/services/ServiceContext/new.md)
- [pool](../../../functions/src/db/Database/pool.md)
- [new](../../../functions/src/llm/tools/registry/ToolRegistry/new.md)
- [register](../../../functions/src/llm/tools/registry/ToolRegistry/register.md)
- [new](../../../functions/src/llm/agent/service/AgentService/new.md)
- [with_tool_registry](../../../functions/src/llm/agent/service/AgentService/with_tool_registry.md)

# Called by

- [test_error_api_error](../../../functions/tests/error_scenarios_test/test_error_api_error.md)
- [test_error_rate_limit](../../../functions/tests/error_scenarios_test/test_error_rate_limit.md)
- [test_error_timeout](../../../functions/tests/error_scenarios_test/test_error_timeout.md)
- [test_error_invalid_response](../../../functions/tests/error_scenarios_test/test_error_invalid_response.md)
- [test_error_authentication](../../../functions/tests/error_scenarios_test/test_error_authentication.md)
- [test_error_session_not_found](../../../functions/tests/error_scenarios_test/test_error_session_not_found.md)
- [test_error_recovery_after_failure](../../../functions/tests/error_scenarios_test/test_error_recovery_after_failure.md)