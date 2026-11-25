---
type: Rust Method
title: get
resource: src/llm/tools/cache.rs#L89-L97
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/agent/service/tool_round/AgentService/execute_tool_round
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/cache/cache_hit_returns_same_result
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/cache/cache_expires_after_ttl
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

`pub fn get(&self, key: &CacheKey) -> Option<String>`

# Called by

- [execute_tool_round](../../../../../../functions/src/llm/agent/service/tool_round/AgentService/execute_tool_round.md)
- [cache_hit_returns_same_result](../../../../../../functions/src/llm/tools/cache/cache_hit_returns_same_result.md)
- [cache_expires_after_ttl](../../../../../../functions/src/llm/tools/cache/cache_expires_after_ttl.md)
- [invalidate_matching_drops_selected_tools_and_keeps_others](../../../../../../functions/src/llm/tools/cache/invalidate_matching_drops_selected_tools_and_keeps_others.md)
- [zero_ttl_insert_is_noop](../../../../../../functions/src/llm/tools/cache/zero_ttl_insert_is_noop.md)