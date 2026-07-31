---
type: Rust Method
title: with_max_tool_iterations
resource: src/llm/agent/service.rs#L573-L576
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/cli/cmd_chat
  - functions/src/cli/cmd_run
  - functions/src/llm/agent/service/AgentServiceLauncher/crate-llm-tools-subagentlauncher/launch
---

# Signature

`pub fn with_max_tool_iterations(mut self, max: usize) -> Self`

# Called by

- [cmd_chat](../../../../../../functions/src/cli/cmd_chat.md)
- [cmd_run](../../../../../../functions/src/cli/cmd_run.md)
- [launch](../../../../../../functions/src/llm/agent/service/AgentServiceLauncher/crate-llm-tools-subagentlauncher/launch.md)