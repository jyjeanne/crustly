---
type: Rust Function
title: create_test_agent
resource: tests/integration_test.rs#L114-L130
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/tests/integration_test/MockProvider/new
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
  - target: functions/tests/integration_test/test_end_to_end_simple_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/integration_test/test_end_to_end_multi_turn_conversation
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/integration_test/test_end_to_end_session_management
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/integration_test/test_end_to_end_cost_tracking
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/integration_test/test_end_to_end_error_handling
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/integration_test/test_end_to_end_token_usage
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn create_test_agent( db: &Database, responses: Vec<String>, ) -> Result<(AgentService, ServiceContext)>`

# Calls

- [new](../../../functions/tests/integration_test/MockProvider/new.md)
- [new](../../../functions/src/services/ServiceContext/new.md)
- [pool](../../../functions/src/db/Database/pool.md)
- [new](../../../functions/src/llm/tools/registry/ToolRegistry/new.md)
- [register](../../../functions/src/llm/tools/registry/ToolRegistry/register.md)
- [new](../../../functions/src/llm/agent/service/AgentService/new.md)
- [with_tool_registry](../../../functions/src/llm/agent/service/AgentService/with_tool_registry.md)

# Called by

- [test_end_to_end_simple_message](../../../functions/tests/integration_test/test_end_to_end_simple_message.md)
- [test_end_to_end_multi_turn_conversation](../../../functions/tests/integration_test/test_end_to_end_multi_turn_conversation.md)
- [test_end_to_end_session_management](../../../functions/tests/integration_test/test_end_to_end_session_management.md)
- [test_end_to_end_cost_tracking](../../../functions/tests/integration_test/test_end_to_end_cost_tracking.md)
- [test_end_to_end_error_handling](../../../functions/tests/integration_test/test_end_to_end_error_handling.md)
- [test_end_to_end_token_usage](../../../functions/tests/integration_test/test_end_to_end_token_usage.md)