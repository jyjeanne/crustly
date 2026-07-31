---
type: Rust Method
title: from_tool
resource: src/llm/tools/cache.rs#L17-L25
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/tui/markdown/MarkdownRenderer/finish
  called_by:
  - functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
  - functions/src/llm/tools/cache/cache_hit_returns_same_result
  - functions/src/llm/tools/cache/cache_expires_after_ttl
  - functions/src/llm/tools/cache/invalidate_matching_drops_selected_tools_and_keeps_others
  - functions/src/llm/tools/cache/zero_ttl_insert_is_noop
---

# Signature

`pub fn from_tool(tool_name: &str, inputs: &Value) -> Self`

# Calls

- [finish](../../../../../../functions/src/tui/markdown/MarkdownRenderer/finish.md)

# Called by

- [send_message_with_tools_inner](../../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)
- [cache_hit_returns_same_result](../../../../../../functions/src/llm/tools/cache/cache_hit_returns_same_result.md)
- [cache_expires_after_ttl](../../../../../../functions/src/llm/tools/cache/cache_expires_after_ttl.md)
- [invalidate_matching_drops_selected_tools_and_keeps_others](../../../../../../functions/src/llm/tools/cache/invalidate_matching_drops_selected_tools_and_keeps_others.md)
- [zero_ttl_insert_is_noop](../../../../../../functions/src/llm/tools/cache/zero_ttl_insert_is_noop.md)