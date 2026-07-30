---
type: Rust Method
title: send_message_with_tools
resource: src/llm/agent/service.rs#L777-L785
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/agent/service/AgentService/send_message_with_tools_and_mode
  called_by:
  - functions/src/cli/cmd_run
  - functions/src/llm/agent/service/AgentServiceLauncher/crate-llm-tools-subagentlauncher/launch
  - functions/src/llm/agent/service/test_send_message_with_tool_execution
---

# Signature

`pub async fn send_message_with_tools( &self, session_id: Uuid, user_message: String, model: Option<String>, ) -> Result<AgentResponse>`

# Calls

- [send_message_with_tools_and_mode](../../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_and_mode.md)

# Called by

- [cmd_run](../../../../../../functions/src/cli/cmd_run.md)
- [launch](../../../../../../functions/src/llm/agent/service/AgentServiceLauncher/crate-llm-tools-subagentlauncher/launch.md)
- [test_send_message_with_tool_execution](../../../../../../functions/src/llm/agent/service/test_send_message_with_tool_execution.md)