---
type: Rust Method
title: canonical_name
resource: src/llm/tools/registry.rs#L118-L126
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/tools/registry/ToolRegistry/is_trusted
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/registry/ToolRegistry/is_denied
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/registry/ToolRegistry/resolve_name
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/registry/ToolRegistry/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/registry/ToolRegistry/has_tool
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/registry/ToolRegistry/execute
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn canonical_name<'a>(&self, name: &'a str) -> &'a str`

# Called by

- [is_trusted](../../../../../../functions/src/llm/tools/registry/ToolRegistry/is_trusted.md)
- [is_denied](../../../../../../functions/src/llm/tools/registry/ToolRegistry/is_denied.md)
- [resolve_name](../../../../../../functions/src/llm/tools/registry/ToolRegistry/resolve_name.md)
- [get](../../../../../../functions/src/llm/tools/registry/ToolRegistry/get.md)
- [has_tool](../../../../../../functions/src/llm/tools/registry/ToolRegistry/has_tool.md)
- [execute](../../../../../../functions/src/llm/tools/registry/ToolRegistry/execute.md)