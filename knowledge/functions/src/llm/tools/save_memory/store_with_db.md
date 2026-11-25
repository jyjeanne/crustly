---
type: Rust Function
title: store_with_db
resource: src/llm/tools/save_memory.rs#L490-L498
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/Database/connect_in_memory
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/Database/run_migrations
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/Database/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/MemoryConfig/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/llm/tools/save_memory/database_save_is_stored_announced_and_not_written_to_the_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/save_memory/database_save_can_be_global_with_a_category
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/save_memory/database_save_refuses_episodes_bad_expiry_and_secrets
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/save_memory/database_replace_with_an_unknown_id_is_an_error
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/save_memory/only_global_saves_with_a_store_need_approval
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn store_with_db() -> (crate::db::Database, Arc<MemoryStore>)`

# Calls

- [connect_in_memory](../../../../../functions/src/db/Database/connect_in_memory.md)
- [run_migrations](../../../../../functions/src/db/Database/run_migrations.md)
- [new](../../../../../functions/src/memory/MemoryStore/new.md)
- [pool](../../../../../functions/src/db/Database/pool.md)
- [default](../../../../../functions/src/config/MemoryConfig/default/default.md)

# Called by

- [database_save_is_stored_announced_and_not_written_to_the_file](../../../../../functions/src/llm/tools/save_memory/database_save_is_stored_announced_and_not_written_to_the_file.md)
- [database_save_can_be_global_with_a_category](../../../../../functions/src/llm/tools/save_memory/database_save_can_be_global_with_a_category.md)
- [database_save_refuses_episodes_bad_expiry_and_secrets](../../../../../functions/src/llm/tools/save_memory/database_save_refuses_episodes_bad_expiry_and_secrets.md)
- [database_replace_with_an_unknown_id_is_an_error](../../../../../functions/src/llm/tools/save_memory/database_replace_with_an_unknown_id_is_an_error.md)
- [only_global_saves_with_a_store_need_approval](../../../../../functions/src/llm/tools/save_memory/only_global_saves_with_a_store_need_approval.md)