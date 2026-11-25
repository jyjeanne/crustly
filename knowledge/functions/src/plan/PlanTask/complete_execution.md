---
type: Rust Method
title: complete_execution
resource: src/plan/mod.rs#L616-L635
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/tools/plan_tool/PlanTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn complete_execution(&mut self, output: String, success: bool)`

# Called by

- [execute](../../../../functions/src/llm/tools/plan_tool/PlanTool/tool/execute.md)