---
type: Rust Method
title: complexity_stars
resource: src/plan/mod.rs#L705-L709
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/tools/plan_tool/PlanTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/plan_tests/test_task_complexity_stars
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_plan_task_lines
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn complexity_stars(&self) -> String`

# Called by

- [execute](../../../../functions/src/llm/tools/plan_tool/PlanTool/tool/execute.md)
- [test_task_complexity_stars](../../../../functions/src/plan/plan_tests/test_task_complexity_stars.md)
- [render_plan_task_lines](../../../../functions/src/tui/render/render_plan_task_lines.md)