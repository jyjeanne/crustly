---
type: Rust Method
title: execute
resource: src/llm/tools/code_exec.rs#L128-L265
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/tools/trait/ToolResult/error
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/project/write
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/error/CrustlyError/code
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/tool_round/ToolRound/push
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/trait/ToolResult/success
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>`

# Calls

- [error](../../../../../../../functions/src/llm/tools/trait/ToolResult/error.md)
- [write](../../../../../../../functions/src/memory/project/write.md)
- [code](../../../../../../../functions/src/error/CrustlyError/code.md)
- [push](../../../../../../../functions/src/llm/agent/service/tool_round/ToolRound/push.md)
- [success](../../../../../../../functions/src/llm/tools/trait/ToolResult/success.md)