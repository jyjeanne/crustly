---
type: Rust Function
title: plan_completion_gate_decision_matrix
resource: src/llm/agent/service.rs#L1879-L1944
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/plan_completion_rejection
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/project/write
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/PlanDocument/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/plan/PlanTask/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn plan_completion_gate_decision_matrix()`

# Calls

- [plan_completion_rejection](../../../../../functions/src/llm/agent/service/plan_completion_rejection.md)
- [write](../../../../../functions/src/memory/project/write.md)
- [new](../../../../../functions/src/plan/PlanDocument/new.md)
- [new](../../../../../functions/src/plan/PlanTask/new.md)
- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)