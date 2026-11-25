---
type: Rust Function
title: write_tool_not_cached
resource: src/llm/tools/cache.rs#L167-L172
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/tools/cache/ToolTtlConfig/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/cache/ToolTtlConfig/ttl_for
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn write_tool_not_cached()`

# Calls

- [default](../../../../../functions/src/llm/tools/cache/ToolTtlConfig/default/default.md)
- [ttl_for](../../../../../functions/src/llm/tools/cache/ToolTtlConfig/ttl_for.md)