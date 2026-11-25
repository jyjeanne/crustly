---
type: Rust Method
title: dependencies_satisfied
resource: src/plan/mod.rs#L300-L306
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/all
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/PlanDocument/get_task
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/llm/tools/plan_tool/PlanTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/PlanDocument/ready_tasks
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn dependencies_satisfied(&self, task: &PlanTask) -> bool`

# Calls

- [all](../../../../functions/src/memory/MemoryStore/all.md)
- [get_task](../../../../functions/src/plan/PlanDocument/get_task.md)

# Called by

- [execute](../../../../functions/src/llm/tools/plan_tool/PlanTool/tool/execute.md)
- [ready_tasks](../../../../functions/src/plan/PlanDocument/ready_tasks.md)