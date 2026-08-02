---
type: Rust Method
title: next_executable_task
resource: src/plan/mod.rs#L251-L267
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/llm/tools/plan_tool/PlanTool/tool/execute
---

# Signature

`pub fn next_executable_task(&self) -> Option<&PlanTask>`

# Called by

- [execute](../../../../functions/src/llm/tools/plan_tool/PlanTool/tool/execute.md)