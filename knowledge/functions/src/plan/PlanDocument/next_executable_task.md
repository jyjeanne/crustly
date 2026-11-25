---
type: Rust Method
title: next_executable_task
resource: src/plan/mod.rs#L251-L267
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/find
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/all
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/plan_tool/PlanTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn next_executable_task(&self) -> Option<&PlanTask>`

# Calls

- [find](../../../../functions/src/memory/MemoryStore/find.md)
- [all](../../../../functions/src/memory/MemoryStore/all.md)

# Called by

- [execute](../../../../functions/src/llm/tools/plan_tool/PlanTool/tool/execute.md)