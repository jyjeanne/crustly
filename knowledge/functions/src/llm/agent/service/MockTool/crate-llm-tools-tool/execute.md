---
type: Rust Method
title: execute
resource: src/llm/agent/service.rs#L2254-L2262
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/tools/trait/ToolResult/success
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn execute( &self, _input: serde_json::Value, _context: &crate::llm::tools::ToolExecutionContext, ) -> crate::llm::tools::Result<crate::llm::tools::ToolResult>`

# Calls

- [success](../../../../../../../functions/src/llm/tools/trait/ToolResult/success.md)