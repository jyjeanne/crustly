---
type: Rust Method
title: load
resource: src/llm/tools/task.rs#L167-L175
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/tools/task/TaskStore/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/llm/tools/task/TaskStore/with_lock
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/task/TaskTool/tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn load(path: &Path) -> Result<Self>`

# Calls

- [new](../../../../../../functions/src/llm/tools/task/TaskStore/new.md)

# Called by

- [with_lock](../../../../../../functions/src/llm/tools/task/TaskStore/with_lock.md)
- [execute](../../../../../../functions/src/llm/tools/task/TaskTool/tool/execute.md)