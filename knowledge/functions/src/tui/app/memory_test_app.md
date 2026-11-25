---
type: Rust Function
title: memory_test_app
resource: src/tui/app.rs#L6017-L6031
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
  - target: functions/src/tui/app/App/set_memory
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/app/remember_saves_a_project_memory_and_global_with_the_flag
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/remember_without_text_shows_usage_and_saves_nothing
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/forget_asks_first_and_deletes_only_with_yes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/memory_lists_the_project_and_global_memories
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/a_save_during_a_response_is_announced_in_the_chat
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn memory_test_app( dir: &std::path::Path, ) -> (App, std::sync::Arc<crate::memory::MemoryStore>)`

# Calls

- [connect_in_memory](../../../../functions/src/db/Database/connect_in_memory.md)
- [run_migrations](../../../../functions/src/db/Database/run_migrations.md)
- [new](../../../../functions/src/memory/MemoryStore/new.md)
- [pool](../../../../functions/src/db/Database/pool.md)
- [default](../../../../functions/src/config/MemoryConfig/default/default.md)
- [set_memory](../../../../functions/src/tui/app/App/set_memory.md)

# Called by

- [remember_saves_a_project_memory_and_global_with_the_flag](../../../../functions/src/tui/app/remember_saves_a_project_memory_and_global_with_the_flag.md)
- [remember_without_text_shows_usage_and_saves_nothing](../../../../functions/src/tui/app/remember_without_text_shows_usage_and_saves_nothing.md)
- [forget_asks_first_and_deletes_only_with_yes](../../../../functions/src/tui/app/forget_asks_first_and_deletes_only_with_yes.md)
- [memory_lists_the_project_and_global_memories](../../../../functions/src/tui/app/memory_lists_the_project_and_global_memories.md)
- [a_save_during_a_response_is_announced_in_the_chat](../../../../functions/src/tui/app/a_save_during_a_response_is_announced_in_the_chat.md)