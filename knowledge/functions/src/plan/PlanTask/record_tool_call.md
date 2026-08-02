---
type: Rust Method
title: record_tool_call
resource: src/plan/mod.rs#L609-L613
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/llm/tools/plan_tool/PlanTool/tool/execute
---

# Signature

`pub fn record_tool_call(&mut self, tool_call: ToolCall)`

# Called by

- [execute](../../../../functions/src/llm/tools/plan_tool/PlanTool/tool/execute.md)