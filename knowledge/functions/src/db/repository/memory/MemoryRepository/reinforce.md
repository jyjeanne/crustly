---
type: Rust Method
title: reinforce
resource: src/db/repository/memory.rs#L382-L394
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/db/repository/memory/record_event
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/MemoryStore/save_with
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_repository_test/reinforce_raises_importance_up_to_five
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn reinforce(&self, id: Uuid, actor: &str, now: DateTime<Utc>) -> Result<()>`

# Calls

- [record_event](../../../../../../functions/src/db/repository/memory/record_event.md)

# Called by

- [save_with](../../../../../../functions/src/memory/MemoryStore/save_with.md)
- [reinforce_raises_importance_up_to_five](../../../../../../functions/tests/memory_repository_test/reinforce_raises_importance_up_to_five.md)