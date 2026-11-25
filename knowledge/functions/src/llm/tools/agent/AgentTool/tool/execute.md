---
type: Rust Method
title: execute
resource: src/llm/tools/agent.rs#L132-L213
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/tools/agent/action_of
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/agent/report_result
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/trait/ToolResult/success
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/trait/ToolResult/with_metadata
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/agent/parse_id
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/agent/status_word
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/agent/run_waits_for_the_answer_and_returns_it
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/agent/without_a_launcher_the_tool_says_so
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>`

# Calls

- [action_of](../../../../../../../functions/src/llm/tools/agent/action_of.md)
- [report_result](../../../../../../../functions/src/llm/tools/agent/report_result.md)
- [success](../../../../../../../functions/src/llm/tools/trait/ToolResult/success.md)
- [with_metadata](../../../../../../../functions/src/llm/tools/trait/ToolResult/with_metadata.md)
- [parse_id](../../../../../../../functions/src/llm/tools/agent/parse_id.md)
- [status_word](../../../../../../../functions/src/llm/tools/agent/status_word.md)

# Called by

- [run_waits_for_the_answer_and_returns_it](../../../../../../../functions/src/llm/tools/agent/run_waits_for_the_answer_and_returns_it.md)
- [without_a_launcher_the_tool_says_so](../../../../../../../functions/src/llm/tools/agent/without_a_launcher_the_tool_says_so.md)