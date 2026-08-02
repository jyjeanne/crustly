---
type: Rust Function
title: create_test_agent
resource: tests/integration_test.rs#L114-L130
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/tools/registry/ToolRegistry/register
  - functions/src/llm/agent/service/AgentService/with_tool_registry
  called_by:
  - functions/tests/integration_test/test_end_to_end_simple_message
  - functions/tests/integration_test/test_end_to_end_multi_turn_conversation
  - functions/tests/integration_test/test_end_to_end_session_management
  - functions/tests/integration_test/test_end_to_end_cost_tracking
  - functions/tests/integration_test/test_end_to_end_error_handling
  - functions/tests/integration_test/test_end_to_end_token_usage
---

# Signature

`async fn create_test_agent( db: &Database, responses: Vec<String>, ) -> Result<(AgentService, ServiceContext)>`

# Calls

- [register](../../../functions/src/llm/tools/registry/ToolRegistry/register.md)
- [with_tool_registry](../../../functions/src/llm/agent/service/AgentService/with_tool_registry.md)

# Called by

- [test_end_to_end_simple_message](../../../functions/tests/integration_test/test_end_to_end_simple_message.md)
- [test_end_to_end_multi_turn_conversation](../../../functions/tests/integration_test/test_end_to_end_multi_turn_conversation.md)
- [test_end_to_end_session_management](../../../functions/tests/integration_test/test_end_to_end_session_management.md)
- [test_end_to_end_cost_tracking](../../../functions/tests/integration_test/test_end_to_end_cost_tracking.md)
- [test_end_to_end_error_handling](../../../functions/tests/integration_test/test_end_to_end_error_handling.md)
- [test_end_to_end_token_usage](../../../functions/tests/integration_test/test_end_to_end_token_usage.md)