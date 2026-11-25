---
type: Rust Method
title: release
resource: src/llm/tools/task.rs#L145-L149
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/tools/task/TaskStore/with_lock
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/task/TaskTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn release(&self) -> Result<()>`

# Called by

- [with_lock](../../../../../../functions/src/llm/tools/task/TaskStore/with_lock.md)
- [execute](../../../../../../functions/src/llm/tools/task/TaskTool/tool/execute.md)