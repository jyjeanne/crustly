---
type: Rust Method
title: get_task_by_order
resource: src/plan/mod.rs#L289-L291
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/llm/tools/plan_tool/PlanTool/tool/execute
---

# Signature

`pub fn get_task_by_order(&self, order: usize) -> Option<&PlanTask>`

# Called by

- [execute](../../../../functions/src/llm/tools/plan_tool/PlanTool/tool/execute.md)