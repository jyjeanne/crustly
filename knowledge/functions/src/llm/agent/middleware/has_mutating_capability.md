---
type: Rust Function
title: has_mutating_capability
resource: src/llm/agent/middleware.rs#L142-L151
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/agent/middleware/LoopGuard/roundmiddleware/on_tool_calls
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/tool_round/AgentService/execute_tool_round
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(crate) fn has_mutating_capability(caps: &[ToolCapability]) -> bool`

# Called by

- [on_tool_calls](../../../../../functions/src/llm/agent/middleware/LoopGuard/roundmiddleware/on_tool_calls.md)
- [execute_tool_round](../../../../../functions/src/llm/agent/service/tool_round/AgentService/execute_tool_round.md)