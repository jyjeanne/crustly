---
type: Rust Method
title: get_task_by_order
resource: src/plan/mod.rs#L289-L291
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/find
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/plan_tool/PlanTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn get_task_by_order(&self, order: usize) -> Option<&PlanTask>`

# Calls

- [find](../../../../functions/src/memory/MemoryStore/find.md)

# Called by

- [execute](../../../../functions/src/llm/tools/plan_tool/PlanTool/tool/execute.md)