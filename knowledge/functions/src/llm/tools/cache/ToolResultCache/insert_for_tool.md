---
type: Rust Method
title: insert_for_tool
resource: src/llm/tools/cache.rs#L114-L117
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/tools/cache/ToolTtlConfig/ttl_for
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/cache/ToolResultCache/insert
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/llm/agent/service/tool_round/AgentService/execute_tool_round
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn insert_for_tool(&self, key: CacheKey, result: String)`

# Calls

- [ttl_for](../../../../../../functions/src/llm/tools/cache/ToolTtlConfig/ttl_for.md)
- [insert](../../../../../../functions/src/llm/tools/cache/ToolResultCache/insert.md)

# Called by

- [execute_tool_round](../../../../../../functions/src/llm/agent/service/tool_round/AgentService/execute_tool_round.md)