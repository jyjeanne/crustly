---
type: Rust Method
title: execute
resource: src/mcp/client.rs#L363-L373
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/mcp/client/MCPClient/call_tool
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/trait/ToolResult/success
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/trait/ToolResult/error
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn execute( &self, input: Value, _ctx: &ToolExecutionContext, ) -> crate::llm::tools::Result<ToolResult>`

# Calls

- [call_tool](../../../../../../functions/src/mcp/client/MCPClient/call_tool.md)
- [success](../../../../../../functions/src/llm/tools/trait/ToolResult/success.md)
- [error](../../../../../../functions/src/llm/tools/trait/ToolResult/error.md)