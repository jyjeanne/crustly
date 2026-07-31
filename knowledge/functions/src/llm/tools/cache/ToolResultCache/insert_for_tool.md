---
type: Rust Method
title: insert_for_tool
resource: src/llm/tools/cache.rs#L114-L117
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/tools/cache/ToolTtlConfig/ttl_for
  called_by:
  - functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
---

# Signature

`pub fn insert_for_tool(&self, key: CacheKey, result: String)`

# Calls

- [ttl_for](../../../../../../functions/src/llm/tools/cache/ToolTtlConfig/ttl_for.md)

# Called by

- [send_message_with_tools_inner](../../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)