---
type: Rust Method
title: launch
resource: src/llm/agent/service.rs#L1827-L1875
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/agent/service/AgentService/with_tool_registry
  - functions/src/llm/agent/service/AgentService/with_max_tool_iterations
  - functions/src/llm/agent/service/AgentService/with_allow_sub_agents
  - functions/src/llm/agent/service/AgentService/send_message_with_tools
  called_by:
  - functions/src/llm/agent/service/sub_agent_launcher_does_not_auto_approve_tools
  - functions/src/llm/tools/agent/AgentTool/tool/execute
---

# Signature

`async fn launch( &self, _agent_id: uuid::Uuid, description: &str, prompt: &str, ) -> std::result::Result<(), String>`

# Calls

- [with_tool_registry](../../../../../../../functions/src/llm/agent/service/AgentService/with_tool_registry.md)
- [with_max_tool_iterations](../../../../../../../functions/src/llm/agent/service/AgentService/with_max_tool_iterations.md)
- [with_allow_sub_agents](../../../../../../../functions/src/llm/agent/service/AgentService/with_allow_sub_agents.md)
- [send_message_with_tools](../../../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools.md)

# Called by

- [sub_agent_launcher_does_not_auto_approve_tools](../../../../../../../functions/src/llm/agent/service/sub_agent_launcher_does_not_auto_approve_tools.md)
- [execute](../../../../../../../functions/src/llm/tools/agent/AgentTool/tool/execute.md)