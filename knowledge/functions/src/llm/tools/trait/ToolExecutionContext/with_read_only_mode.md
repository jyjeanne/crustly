---
type: Rust Method
title: with_read_only_mode
resource: src/llm/tools/trait.rs#L176-L179
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/agent/run_waits_for_the_answer_and_returns_it
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/powershell/execute_blocks_dangerous_command_in_read_only_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/powershell/execute_allows_read_only_command_in_plan_mode
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn with_read_only_mode(mut self, read_only: bool) -> Self`

# Called by

- [send_message_with_tools_inner](../../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)
- [run_waits_for_the_answer_and_returns_it](../../../../../../functions/src/llm/tools/agent/run_waits_for_the_answer_and_returns_it.md)
- [execute_blocks_dangerous_command_in_read_only_mode](../../../../../../functions/src/llm/tools/powershell/execute_blocks_dangerous_command_in_read_only_mode.md)
- [execute_allows_read_only_command_in_plan_mode](../../../../../../functions/src/llm/tools/powershell/execute_allows_read_only_command_in_plan_mode.md)