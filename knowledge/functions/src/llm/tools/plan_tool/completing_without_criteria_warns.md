---
type: Rust Function
title: completing_without_criteria_warns
resource: src/llm/tools/plan_tool.rs#L1094-L1134
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/tools/trait/ToolExecutionContext/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/trait/ToolExecutionContext/with_working_directory
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/plan_tool/PlanTool/tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn completing_without_criteria_warns()`

# Calls

- [new](../../../../../functions/src/llm/tools/trait/ToolExecutionContext/new.md)
- [with_working_directory](../../../../../functions/src/llm/tools/trait/ToolExecutionContext/with_working_directory.md)
- [execute](../../../../../functions/src/llm/tools/plan_tool/PlanTool/tool/execute.md)