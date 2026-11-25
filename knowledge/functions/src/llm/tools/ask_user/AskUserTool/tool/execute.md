---
type: Rust Method
title: execute
resource: src/llm/tools/ask_user.rs#L77-L130
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/tools/trait/ToolResult/success
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/trait/ToolResult/error
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/trait/ToolResult/with_metadata
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/ask_user/test_auto_approve_returns_placeholder
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>`

# Calls

- [success](../../../../../../../functions/src/llm/tools/trait/ToolResult/success.md)
- [error](../../../../../../../functions/src/llm/tools/trait/ToolResult/error.md)
- [with_metadata](../../../../../../../functions/src/llm/tools/trait/ToolResult/with_metadata.md)

# Called by

- [test_auto_approve_returns_placeholder](../../../../../../../functions/src/llm/tools/ask_user/test_auto_approve_returns_placeholder.md)