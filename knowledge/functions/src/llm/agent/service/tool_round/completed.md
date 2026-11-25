---
type: Rust Function
title: completed
resource: src/llm/agent/service/tool_round.rs#L72-L111
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/tools/schema_check/describe_arguments
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/service/tool_round/AgentService/execute_tool_round
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn completed( tool_id: String, name: String, input: Value, outcome: std::result::Result<ToolResult, ToolError>, schema: Option<&Value>, ) -> (ContentBlock, ToolCallRecord)`

# Calls

- [describe_arguments](../../../../../../functions/src/llm/tools/schema_check/describe_arguments.md)

# Called by

- [execute_tool_round](../../../../../../functions/src/llm/agent/service/tool_round/AgentService/execute_tool_round.md)