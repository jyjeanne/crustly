---
type: Rust Method
title: acquire
resource: src/llm/tools/task.rs#L75-L142
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/db/Database/run_migrations
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/task/TaskStore/with_lock
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/task/TaskTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_test/secure_delete_is_on_for_every_connection
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn acquire(store_path: &Path) -> Result<Self>`

# Called by

- [run_migrations](../../../../../../functions/src/db/Database/run_migrations.md)
- [with_lock](../../../../../../functions/src/llm/tools/task/TaskStore/with_lock.md)
- [execute](../../../../../../functions/src/llm/tools/task/TaskTool/tool/execute.md)
- [secure_delete_is_on_for_every_connection](../../../../../../functions/tests/memory_test/secure_delete_is_on_for_every_connection.md)