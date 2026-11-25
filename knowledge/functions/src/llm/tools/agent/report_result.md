---
type: Rust Function
title: report_result
resource: src/llm/tools/agent.rs#L249-L279
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/tools/trait/ToolResult/success
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/trait/ToolResult/with_metadata
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/agent/status_word
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/agent/AgentTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/agent/a_failed_report_is_a_failed_tool_call
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn report_result(report: &SubAgentReport) -> ToolResult`

# Calls

- [success](../../../../../functions/src/llm/tools/trait/ToolResult/success.md)
- [with_metadata](../../../../../functions/src/llm/tools/trait/ToolResult/with_metadata.md)
- [status_word](../../../../../functions/src/llm/tools/agent/status_word.md)

# Called by

- [execute](../../../../../functions/src/llm/tools/agent/AgentTool/tool/execute.md)
- [a_failed_report_is_a_failed_tool_call](../../../../../functions/src/llm/tools/agent/a_failed_report_is_a_failed_tool_call.md)