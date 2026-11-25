---
type: Rust Method
title: insert
resource: src/db/repository/memory.rs#L81-L126
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/db/repository/memory/scope_columns
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/retry/retry_db_sqlx
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/model/MemoryCategory/as_str
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/model/MemorySource/as_str
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/model/MemoryStatus/as_str
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/memory/record_event
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/retry/DbRetryConfig/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/reject_refuses_a_memory_that_is_not_pending
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/MemoryStore/save_with
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/app/memory_screen_app
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_consolidate_test/insert_at
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_review_fixes_test/forget_is_refused_in_plan_mode_and_deletes_nothing
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_semantic_test/an_approved_memory_is_embedded
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_vectors_test/insert
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn insert(&self, memory: &InsertMemory, actor: &str) -> Result<()>`

# Calls

- [scope_columns](../../../../../../functions/src/db/repository/memory/scope_columns.md)
- [retry_db_sqlx](../../../../../../functions/src/db/retry/retry_db_sqlx.md)
- [as_str](../../../../../../functions/src/memory/model/MemoryCategory/as_str.md)
- [as_str](../../../../../../functions/src/memory/model/MemorySource/as_str.md)
- [as_str](../../../../../../functions/src/memory/model/MemoryStatus/as_str.md)
- [record_event](../../../../../../functions/src/db/repository/memory/record_event.md)
- [default](../../../../../../functions/src/db/retry/DbRetryConfig/default/default.md)
- [context](../../../../../../functions/src/services/ServiceManager/context.md)

# Called by

- [reject_refuses_a_memory_that_is_not_pending](../../../../../../functions/src/cli/reject_refuses_a_memory_that_is_not_pending.md)
- [save_with](../../../../../../functions/src/memory/MemoryStore/save_with.md)
- [memory_screen_app](../../../../../../functions/src/tui/app/memory_screen_app.md)
- [insert_at](../../../../../../functions/tests/memory_consolidate_test/insert_at.md)
- [forget_is_refused_in_plan_mode_and_deletes_nothing](../../../../../../functions/tests/memory_review_fixes_test/forget_is_refused_in_plan_mode_and_deletes_nothing.md)
- [an_approved_memory_is_embedded](../../../../../../functions/tests/memory_semantic_test/an_approved_memory_is_embedded.md)
- [insert](../../../../../../functions/tests/memory_vectors_test/insert.md)