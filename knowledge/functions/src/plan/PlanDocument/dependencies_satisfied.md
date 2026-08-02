---
type: Rust Method
title: dependencies_satisfied
resource: src/plan/mod.rs#L300-L306
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/llm/tools/plan_tool/PlanTool/tool/execute
  - functions/src/plan/PlanDocument/ready_tasks
---

# Signature

`pub fn dependencies_satisfied(&self, task: &PlanTask) -> bool`

# Called by

- [execute](../../../../functions/src/llm/tools/plan_tool/PlanTool/tool/execute.md)
- [ready_tasks](../../../../functions/src/plan/PlanDocument/ready_tasks.md)