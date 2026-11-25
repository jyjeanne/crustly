---
type: Rust Function
title: db_context
resource: src/llm/tools/save_memory.rs#L480-L488
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/db/Database/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/trait/ToolExecutionContext/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/trait/ToolExecutionContext/with_working_directory
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
---

# Signature

`async fn db_context(db: &crate::db::Database, dir: &TempDir) -> ToolExecutionContext`

# Calls

- [pool](../../../../../functions/src/db/Database/pool.md)
- [new](../../../../../functions/src/llm/tools/trait/ToolExecutionContext/new.md)
- [with_working_directory](../../../../../functions/src/llm/tools/trait/ToolExecutionContext/with_working_directory.md)

# Called by

- [database_save_is_stored_announced_and_not_written_to_the_file](../../../../../functions/src/llm/tools/save_memory/database_save_is_stored_announced_and_not_written_to_the_file.md)
- [database_save_can_be_global_with_a_category](../../../../../functions/src/llm/tools/save_memory/database_save_can_be_global_with_a_category.md)
- [database_save_refuses_episodes_bad_expiry_and_secrets](../../../../../functions/src/llm/tools/save_memory/database_save_refuses_episodes_bad_expiry_and_secrets.md)
- [database_replace_with_an_unknown_id_is_an_error](../../../../../functions/src/llm/tools/save_memory/database_replace_with_an_unknown_id_is_an_error.md)