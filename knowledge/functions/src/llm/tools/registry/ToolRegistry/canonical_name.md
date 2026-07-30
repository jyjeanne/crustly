---
type: Rust Method
title: canonical_name
resource: src/llm/tools/registry.rs#L93-L101
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/llm/tools/registry/ToolRegistry/is_trusted
  - functions/src/llm/tools/registry/ToolRegistry/get
  - functions/src/llm/tools/registry/ToolRegistry/has_tool
  - functions/src/llm/tools/registry/ToolRegistry/execute
---

# Signature

`fn canonical_name<'a>(&self, name: &'a str) -> &'a str`

# Called by

- [is_trusted](../../../../../../functions/src/llm/tools/registry/ToolRegistry/is_trusted.md)
- [get](../../../../../../functions/src/llm/tools/registry/ToolRegistry/get.md)
- [has_tool](../../../../../../functions/src/llm/tools/registry/ToolRegistry/has_tool.md)
- [execute](../../../../../../functions/src/llm/tools/registry/ToolRegistry/execute.md)