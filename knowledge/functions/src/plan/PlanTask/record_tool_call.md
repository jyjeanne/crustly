---
type: Rust Method
title: record_tool_call
resource: src/plan/mod.rs#L609-L613
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/tool_round/ToolRound/push
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/plan_tool/PlanTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn record_tool_call(&mut self, tool_call: ToolCall)`

# Calls

- [push](../../../../functions/src/llm/agent/service/tool_round/ToolRound/push.md)

# Called by

- [execute](../../../../functions/src/llm/tools/plan_tool/PlanTool/tool/execute.md)