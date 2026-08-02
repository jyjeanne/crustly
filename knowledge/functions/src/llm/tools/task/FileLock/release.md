---
type: Rust Method
title: release
resource: src/llm/tools/task.rs#L145-L149
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/llm/tools/task/TaskStore/with_lock
  - functions/src/llm/tools/task/TaskTool/tool/execute
---

# Signature

`async fn release(&self) -> Result<()>`

# Called by

- [with_lock](../../../../../../functions/src/llm/tools/task/TaskStore/with_lock.md)
- [execute](../../../../../../functions/src/llm/tools/task/TaskTool/tool/execute.md)