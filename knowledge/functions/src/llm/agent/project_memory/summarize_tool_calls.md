---
type: Rust Function
title: summarize_tool_calls
resource: src/llm/agent/project_memory.rs#L107-L131
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/project_memory/call_target
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/tool_round/ToolRound/push
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/project_memory/tool_calls_are_summarized_with_their_targets
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/project_memory/long_requests_keep_the_first_calls_and_count_the_rest
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn summarize_tool_calls(records: &[ToolCallRecord]) -> Option<String>`

# Calls

- [call_target](../../../../../functions/src/llm/agent/project_memory/call_target.md)
- [push](../../../../../functions/src/llm/agent/service/tool_round/ToolRound/push.md)

# Called by

- [tool_calls_are_summarized_with_their_targets](../../../../../functions/src/llm/agent/project_memory/tool_calls_are_summarized_with_their_targets.md)
- [long_requests_keep_the_first_calls_and_count_the_rest](../../../../../functions/src/llm/agent/project_memory/long_requests_keep_the_first_calls_and_count_the_rest.md)
- [send_message_with_tools_inner](../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)