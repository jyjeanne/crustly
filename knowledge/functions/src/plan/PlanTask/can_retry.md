---
type: Rust Method
title: can_retry
resource: src/plan/mod.rs#L668-L671
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/tools/plan_tool/PlanTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/PlanDocument/retriable_tasks
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn can_retry(&self) -> bool`

# Called by

- [execute](../../../../functions/src/llm/tools/plan_tool/PlanTool/tool/execute.md)
- [retriable_tasks](../../../../functions/src/plan/PlanDocument/retriable_tasks.md)