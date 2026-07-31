---
type: Rust Method
title: execute
resource: src/llm/tools/registry.rs#L139-L220
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/tools/registry/ToolRegistry/canonical_name
---

# Signature

`pub async fn execute( &self, name: &str, input: Value, context: &ToolExecutionContext, ) -> Result<ToolResult>`

# Calls

- [canonical_name](../../../../../../functions/src/llm/tools/registry/ToolRegistry/canonical_name.md)