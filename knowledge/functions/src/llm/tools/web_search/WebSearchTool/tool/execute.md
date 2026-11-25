---
type: Rust Method
title: execute
resource: src/llm/tools/web_search.rs#L129-L225
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/subagents/ChildTemplate/build
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama_models/PullProgress/is_success
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/trait/ToolResult/error
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/trait/ToolResult/success
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/vector/encode
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn execute(&self, input: Value, _context: &ToolExecutionContext) -> Result<ToolResult>`

# Calls

- [build](../../../../../../../functions/src/llm/agent/service/subagents/ChildTemplate/build.md)
- [is_success](../../../../../../../functions/src/llm/provider/ollama_models/PullProgress/is_success.md)
- [error](../../../../../../../functions/src/llm/tools/trait/ToolResult/error.md)
- [success](../../../../../../../functions/src/llm/tools/trait/ToolResult/success.md)
- [encode](../../../../../../../functions/src/memory/vector/encode.md)