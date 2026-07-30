---
type: Rust Method
title: get_task_by_order_mut
resource: src/plan/mod.rs#L294-L297
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/llm/tools/plan_tool/PlanTool/tool/execute
---

# Signature

`pub fn get_task_by_order_mut(&mut self, order: usize) -> Option<&mut PlanTask>`

# Called by

- [execute](../../../../functions/src/llm/tools/plan_tool/PlanTool/tool/execute.md)