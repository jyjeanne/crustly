---
type: Rust Function
title: configure_agent_loop
resource: src/cli/mod.rs#L235-L254
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/AgentService/with_core_system_prompt
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/with_tool_profile
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/with_max_tool_iterations
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/with_request_limits
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/AgentConfig/max_wall_time
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/AgentConfig/max_request_tokens
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/with_verification
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/with_project_notes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/with_progress_file
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/configure_agent
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/cmd_chat
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn configure_agent_loop( service: crate::llm::agent::AgentService, config: &crate::config::Config, can_approve_shell: bool, ) -> crate::llm::agent::AgentService`

# Calls

- [with_core_system_prompt](../../../functions/src/llm/agent/service/AgentService/with_core_system_prompt.md)
- [with_tool_profile](../../../functions/src/llm/agent/service/AgentService/with_tool_profile.md)
- [with_max_tool_iterations](../../../functions/src/llm/agent/service/AgentService/with_max_tool_iterations.md)
- [with_request_limits](../../../functions/src/llm/agent/service/AgentService/with_request_limits.md)
- [max_wall_time](../../../functions/src/config/AgentConfig/max_wall_time.md)
- [max_request_tokens](../../../functions/src/config/AgentConfig/max_request_tokens.md)
- [with_verification](../../../functions/src/llm/agent/service/AgentService/with_verification.md)
- [with_project_notes](../../../functions/src/llm/agent/service/AgentService/with_project_notes.md)
- [with_progress_file](../../../functions/src/llm/agent/service/AgentService/with_progress_file.md)

# Called by

- [configure_agent](../../../functions/src/cli/configure_agent.md)
- [cmd_chat](../../../functions/src/cli/cmd_chat.md)