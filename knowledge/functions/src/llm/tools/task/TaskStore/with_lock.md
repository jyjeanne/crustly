---
type: Rust Method
title: with_lock
resource: src/llm/tools/task.rs#L197-L217
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/tools/task/FileLock/acquire
  - functions/src/llm/tools/task/FileLock/release
  called_by:
  - functions/src/llm/tools/task/TaskTool/tool/execute
---

# Signature

`async fn with_lock<F, T>(path: &Path, operation: F) -> Result<T> where F: FnOnce(&mut Self) -> Result<T>,`

# Calls

- [acquire](../../../../../../functions/src/llm/tools/task/FileLock/acquire.md)
- [release](../../../../../../functions/src/llm/tools/task/FileLock/release.md)

# Called by

- [execute](../../../../../../functions/src/llm/tools/task/TaskTool/tool/execute.md)