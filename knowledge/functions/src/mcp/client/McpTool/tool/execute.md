---
type: Rust Method
title: execute
resource: src/mcp/client.rs#L361-L371
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/mcp/client/MCPClient/call_tool
---

# Signature

`async fn execute( &self, input: Value, _ctx: &ToolExecutionContext, ) -> crate::llm::tools::Result<ToolResult>`

# Calls

- [call_tool](../../../../../../functions/src/mcp/client/MCPClient/call_tool.md)