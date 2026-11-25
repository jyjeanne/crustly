---
type: Rust Method
title: next_executable_task_mut
resource: src/plan/mod.rs#L270-L286
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/find
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/all
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn next_executable_task_mut(&mut self) -> Option<&mut PlanTask>`

# Calls

- [find](../../../../functions/src/memory/MemoryStore/find.md)
- [all](../../../../functions/src/memory/MemoryStore/all.md)