---
type: Rust Method
title: with_lock
resource: src/llm/tools/task.rs#L197-L217
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/tools/task/FileLock/acquire
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/task/TaskStore/load
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/task/TaskStore/save
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/task/FileLock/release
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/task/TaskTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn with_lock<F, T>(path: &Path, operation: F) -> Result<T> where F: FnOnce(&mut Self) -> Result<T>,`

# Calls

- [acquire](../../../../../../functions/src/llm/tools/task/FileLock/acquire.md)
- [load](../../../../../../functions/src/llm/tools/task/TaskStore/load.md)
- [save](../../../../../../functions/src/llm/tools/task/TaskStore/save.md)
- [release](../../../../../../functions/src/llm/tools/task/FileLock/release.md)

# Called by

- [execute](../../../../../../functions/src/llm/tools/task/TaskTool/tool/execute.md)