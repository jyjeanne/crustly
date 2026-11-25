---
type: Rust Method
title: save
resource: src/llm/tools/task.rs#L177-L194
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/project/write
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/task/TaskStore/with_lock
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn save(&self, path: &Path) -> Result<()>`

# Calls

- [write](../../../../../../functions/src/memory/project/write.md)

# Called by

- [with_lock](../../../../../../functions/src/llm/tools/task/TaskStore/with_lock.md)