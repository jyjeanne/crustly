---
type: Rust Method
title: execute
resource: src/llm/tools/search_memory.rs#L114-L152
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/tools/trait/ToolResult/error
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/model/MemoryCategory/fromstr/from_str
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/project/identify
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/search
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/trait/ToolResult/success
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>`

# Calls

- [error](../../../../../../../functions/src/llm/tools/trait/ToolResult/error.md)
- [from_str](../../../../../../../functions/src/memory/model/MemoryCategory/fromstr/from_str.md)
- [identify](../../../../../../../functions/src/memory/project/identify.md)
- [search](../../../../../../../functions/src/memory/MemoryStore/search.md)
- [success](../../../../../../../functions/src/llm/tools/trait/ToolResult/success.md)