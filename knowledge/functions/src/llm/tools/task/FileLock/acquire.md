---
type: Rust Method
title: acquire
resource: src/llm/tools/task.rs#L75-L142
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/db/Database/run_migrations
  - functions/src/llm/tools/task/TaskStore/with_lock
  - functions/src/llm/tools/task/TaskTool/tool/execute
---

# Signature

`async fn acquire(store_path: &Path) -> Result<Self>`

# Called by

- [run_migrations](../../../../../../functions/src/db/Database/run_migrations.md)
- [with_lock](../../../../../../functions/src/llm/tools/task/TaskStore/with_lock.md)
- [execute](../../../../../../functions/src/llm/tools/task/TaskTool/tool/execute.md)