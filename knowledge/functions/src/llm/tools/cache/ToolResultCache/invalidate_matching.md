---
type: Rust Method
title: invalidate_matching
resource: src/llm/tools/cache.rs#L132-L134
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/agent/service/tool_round/AgentService/execute_tool_round
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/cache/invalidate_matching_drops_selected_tools_and_keeps_others
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn invalidate_matching(&self, pred: impl Fn(&str) -> bool)`

# Called by

- [execute_tool_round](../../../../../../functions/src/llm/agent/service/tool_round/AgentService/execute_tool_round.md)
- [invalidate_matching_drops_selected_tools_and_keeps_others](../../../../../../functions/src/llm/tools/cache/invalidate_matching_drops_selected_tools_and_keeps_others.md)