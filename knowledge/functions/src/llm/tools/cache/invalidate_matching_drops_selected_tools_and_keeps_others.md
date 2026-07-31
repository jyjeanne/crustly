---
type: Rust Function
title: invalidate_matching_drops_selected_tools_and_keeps_others
resource: src/llm/tools/cache.rs#L175-L189
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/tools/cache/CacheKey/from_tool
  - functions/src/llm/tools/cache/ToolResultCache/invalidate_matching
---

# Signature

`fn invalidate_matching_drops_selected_tools_and_keeps_others()`

# Calls

- [from_tool](../../../../../functions/src/llm/tools/cache/CacheKey/from_tool.md)
- [invalidate_matching](../../../../../functions/src/llm/tools/cache/ToolResultCache/invalidate_matching.md)