---
type: Rust Method
title: execute
resource: src/llm/tools/registry.rs#L367-L373
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

`async fn execute( &self, _input: Value, _context: &ToolExecutionContext, ) -> Result<ToolResult>`

# Calls

- [success](../../../../../../../functions/src/llm/tools/trait/ToolResult/success.md)