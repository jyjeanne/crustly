---
type: Rust Function
title: create_error_agent
resource: tests/error_scenarios_test.rs#L94-L110
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/tools/registry/ToolRegistry/register
  - functions/src/llm/agent/service/AgentService/with_tool_registry
  called_by:
  - functions/tests/error_scenarios_test/test_error_api_error
  - functions/tests/error_scenarios_test/test_error_rate_limit
  - functions/tests/error_scenarios_test/test_error_timeout
  - functions/tests/error_scenarios_test/test_error_invalid_response
  - functions/tests/error_scenarios_test/test_error_authentication
  - functions/tests/error_scenarios_test/test_error_session_not_found
  - functions/tests/error_scenarios_test/test_error_recovery_after_failure
---

# Signature

`async fn create_error_agent( db: &Database, error_type: ErrorType, ) -> Result<(AgentService, ServiceContext)>`

# Calls

- [register](../../../functions/src/llm/tools/registry/ToolRegistry/register.md)
- [with_tool_registry](../../../functions/src/llm/agent/service/AgentService/with_tool_registry.md)

# Called by

- [test_error_api_error](../../../functions/tests/error_scenarios_test/test_error_api_error.md)
- [test_error_rate_limit](../../../functions/tests/error_scenarios_test/test_error_rate_limit.md)
- [test_error_timeout](../../../functions/tests/error_scenarios_test/test_error_timeout.md)
- [test_error_invalid_response](../../../functions/tests/error_scenarios_test/test_error_invalid_response.md)
- [test_error_authentication](../../../functions/tests/error_scenarios_test/test_error_authentication.md)
- [test_error_session_not_found](../../../functions/tests/error_scenarios_test/test_error_session_not_found.md)
- [test_error_recovery_after_failure](../../../functions/tests/error_scenarios_test/test_error_recovery_after_failure.md)