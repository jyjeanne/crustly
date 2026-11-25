---
type: Rust Function
title: parse_status
resource: src/llm/tools/task.rs#L286-L298
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/tools/task/TaskTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn parse_status(status_str: &str) -> Result<TaskStatus>`

# Called by

- [execute](../../../../../functions/src/llm/tools/task/TaskTool/tool/execute.md)