---
type: Rust Function
title: zero_ttl_insert_is_noop
resource: src/llm/tools/cache.rs#L192-L197
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/tools/cache/ToolResultCache/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/cache/ToolTtlConfig/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/cache/CacheKey/from_tool
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/cache/ToolResultCache/insert
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/cache/ToolResultCache/get
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn zero_ttl_insert_is_noop()`

# Calls

- [new](../../../../../functions/src/llm/tools/cache/ToolResultCache/new.md)
- [default](../../../../../functions/src/llm/tools/cache/ToolTtlConfig/default/default.md)
- [from_tool](../../../../../functions/src/llm/tools/cache/CacheKey/from_tool.md)
- [insert](../../../../../functions/src/llm/tools/cache/ToolResultCache/insert.md)
- [get](../../../../../functions/src/llm/tools/cache/ToolResultCache/get.md)