---
type: Rust Method
title: with_read_only_mode
resource: src/llm/tools/trait.rs#L100-L103
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
  - functions/src/llm/tools/powershell/execute_blocks_dangerous_command_in_read_only_mode
  - functions/src/llm/tools/powershell/execute_allows_read_only_command_in_plan_mode
---

# Signature

`pub fn with_read_only_mode(mut self, read_only: bool) -> Self`

# Called by

- [send_message_with_tools_inner](../../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)
- [execute_blocks_dangerous_command_in_read_only_mode](../../../../../../functions/src/llm/tools/powershell/execute_blocks_dangerous_command_in_read_only_mode.md)
- [execute_allows_read_only_command_in_plan_mode](../../../../../../functions/src/llm/tools/powershell/execute_allows_read_only_command_in_plan_mode.md)