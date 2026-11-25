---
type: Rust Method
title: execute
resource: src/llm/tools/context.rs#L210-L413
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/tools/context/ContextStore/load
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/context/ContextStore/save
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/trait/ToolResult/success
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/tool_round/ToolRound/push
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/trait/ToolResult/error
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>`

# Calls

- [load](../../../../../../../functions/src/llm/tools/context/ContextStore/load.md)
- [save](../../../../../../../functions/src/llm/tools/context/ContextStore/save.md)
- [contains](../../../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)
- [success](../../../../../../../functions/src/llm/tools/trait/ToolResult/success.md)
- [push](../../../../../../../functions/src/llm/agent/service/tool_round/ToolRound/push.md)
- [error](../../../../../../../functions/src/llm/tools/trait/ToolResult/error.md)