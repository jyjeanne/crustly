---
type: Rust Method
title: with_tool_registry
resource: src/llm/agent/service.rs#L579-L582
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/cli/cmd_chat
  - functions/src/cli/cmd_run
  - functions/src/llm/agent/service/AgentServiceLauncher/crate-llm-tools-subagentlauncher/launch
  - functions/src/llm/agent/service/test_send_message_with_tool_execution
  - functions/tests/error_scenarios_test/create_error_agent
  - functions/tests/integration_test/create_test_agent
---

# Signature

`pub fn with_tool_registry(mut self, registry: Arc<ToolRegistry>) -> Self`

# Called by

- [cmd_chat](../../../../../../functions/src/cli/cmd_chat.md)
- [cmd_run](../../../../../../functions/src/cli/cmd_run.md)
- [launch](../../../../../../functions/src/llm/agent/service/AgentServiceLauncher/crate-llm-tools-subagentlauncher/launch.md)
- [test_send_message_with_tool_execution](../../../../../../functions/src/llm/agent/service/test_send_message_with_tool_execution.md)
- [create_error_agent](../../../../../../functions/tests/error_scenarios_test/create_error_agent.md)
- [create_test_agent](../../../../../../functions/tests/integration_test/create_test_agent.md)