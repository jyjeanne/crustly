---
type: Rust Method
title: default
resource: src/llm/tools/cache.rs#L47-L57
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/agent/service/AgentService/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/cache/cache_hit_returns_same_result
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/cache/cache_expires_after_ttl
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/cache/write_tool_not_cached
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/cache/invalidate_matching_drops_selected_tools_and_keeps_others
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/cache/zero_ttl_insert_is_noop
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn default() -> Self`

# Called by

- [new](../../../../../../../functions/src/llm/agent/service/AgentService/new.md)
- [cache_hit_returns_same_result](../../../../../../../functions/src/llm/tools/cache/cache_hit_returns_same_result.md)
- [cache_expires_after_ttl](../../../../../../../functions/src/llm/tools/cache/cache_expires_after_ttl.md)
- [write_tool_not_cached](../../../../../../../functions/src/llm/tools/cache/write_tool_not_cached.md)
- [invalidate_matching_drops_selected_tools_and_keeps_others](../../../../../../../functions/src/llm/tools/cache/invalidate_matching_drops_selected_tools_and_keeps_others.md)
- [zero_ttl_insert_is_noop](../../../../../../../functions/src/llm/tools/cache/zero_ttl_insert_is_noop.md)