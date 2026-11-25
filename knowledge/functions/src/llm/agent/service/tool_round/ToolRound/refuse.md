---
type: Rust Method
title: refuse
resource: src/llm/agent/service/tool_round.rs#L44-L46
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/tool_round/ToolRound/push
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/tool_round/refused
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/service/tool_round/AgentService/execute_tool_round
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn refuse(&mut self, tool_id: String, name: String, input: Value, message: String)`

# Calls

- [push](../../../../../../../functions/src/llm/agent/service/tool_round/ToolRound/push.md)
- [refused](../../../../../../../functions/src/llm/agent/service/tool_round/refused.md)

# Called by

- [execute_tool_round](../../../../../../../functions/src/llm/agent/service/tool_round/AgentService/execute_tool_round.md)