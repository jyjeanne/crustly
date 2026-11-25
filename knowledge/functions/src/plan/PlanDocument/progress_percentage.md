---
type: Rust Method
title: progress_percentage
resource: src/plan/mod.rs#L164-L170
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/plan/PlanDocument/count_by_status
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/plan_tool/PlanTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/plan_tests/test_progress_percentage
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/plan/PlanService/get_statistics
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn progress_percentage(&self) -> f32`

# Calls

- [count_by_status](../../../../functions/src/plan/PlanDocument/count_by_status.md)

# Called by

- [execute](../../../../functions/src/llm/tools/plan_tool/PlanTool/tool/execute.md)
- [test_progress_percentage](../../../../functions/src/plan/plan_tests/test_progress_percentage.md)
- [get_statistics](../../../../functions/src/services/plan/PlanService/get_statistics.md)