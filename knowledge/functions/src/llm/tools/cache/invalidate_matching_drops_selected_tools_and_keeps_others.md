---
type: Rust Function
title: invalidate_matching_drops_selected_tools_and_keeps_others
resource: src/llm/tools/cache.rs#L175-L189
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
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
  - target: functions/src/llm/tools/cache/ToolResultCache/invalidate_matching
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/cache/ToolResultCache/get
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn invalidate_matching_drops_selected_tools_and_keeps_others()`

# Calls

- [new](../../../../../functions/src/llm/tools/cache/ToolResultCache/new.md)
- [default](../../../../../functions/src/llm/tools/cache/ToolTtlConfig/default/default.md)
- [from_tool](../../../../../functions/src/llm/tools/cache/CacheKey/from_tool.md)
- [insert](../../../../../functions/src/llm/tools/cache/ToolResultCache/insert.md)
- [invalidate_matching](../../../../../functions/src/llm/tools/cache/ToolResultCache/invalidate_matching.md)
- [get](../../../../../functions/src/llm/tools/cache/ToolResultCache/get.md)