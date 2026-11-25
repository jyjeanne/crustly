---
type: Rust Method
title: execute
resource: src/llm/agent/service.rs#L3499-L3512
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

`async fn execute( &self, input: serde_json::Value, _context: &ToolExecutionContext, ) -> crate::llm::tools::Result<crate::llm::tools::ToolResult>`

# Calls

- [success](../../../../../../../functions/src/llm/tools/trait/ToolResult/success.md)