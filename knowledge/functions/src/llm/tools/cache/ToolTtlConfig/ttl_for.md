---
type: Rust Method
title: ttl_for
resource: src/llm/tools/cache.rs#L61-L71
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/tools/cache/ToolResultCache/insert_for_tool
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/cache/write_tool_not_cached
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn ttl_for(&self, tool_name: &str) -> Duration`

# Called by

- [insert_for_tool](../../../../../../functions/src/llm/tools/cache/ToolResultCache/insert_for_tool.md)
- [write_tool_not_cached](../../../../../../functions/src/llm/tools/cache/write_tool_not_cached.md)