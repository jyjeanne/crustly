---
type: Rust Function
title: memory_screen_app
resource: src/tui/app.rs#L6155-L6226
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
  - target: functions/src/memory/project/identify
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/register_project
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/save
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/memory/MemoryRepository/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/memory/MemoryRepository/insert
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/tui/app/memories_opens_the_screen_with_pending_memories_first
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/scope_and_category_filters_narrow_the_list
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/search_filters_by_words_in_the_content
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/approve_makes_a_pending_memory_active
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/reject_deletes_a_pending_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/forget_asks_first_and_deletes_only_on_y
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/enter_shows_details_with_the_history
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn memory_screen_app( dir: &std::path::Path, ) -> ( App, std::sync::Arc<crate::memory::MemoryStore>, crate::db::Database, )`

# Calls

- [connect_in_memory](../../../../functions/src/db/Database/connect_in_memory.md)
- [run_migrations](../../../../functions/src/db/Database/run_migrations.md)
- [new](../../../../functions/src/memory/MemoryStore/new.md)
- [pool](../../../../functions/src/db/Database/pool.md)
- [default](../../../../functions/src/config/MemoryConfig/default/default.md)
- [set_memory](../../../../functions/src/tui/app/App/set_memory.md)
- [identify](../../../../functions/src/memory/project/identify.md)
- [register_project](../../../../functions/src/memory/MemoryStore/register_project.md)
- [save](../../../../functions/src/memory/MemoryStore/save.md)
- [new](../../../../functions/src/db/repository/memory/MemoryRepository/new.md)
- [insert](../../../../functions/src/db/repository/memory/MemoryRepository/insert.md)

# Called by

- [memories_opens_the_screen_with_pending_memories_first](../../../../functions/src/tui/app/memories_opens_the_screen_with_pending_memories_first.md)
- [scope_and_category_filters_narrow_the_list](../../../../functions/src/tui/app/scope_and_category_filters_narrow_the_list.md)
- [search_filters_by_words_in_the_content](../../../../functions/src/tui/app/search_filters_by_words_in_the_content.md)
- [approve_makes_a_pending_memory_active](../../../../functions/src/tui/app/approve_makes_a_pending_memory_active.md)
- [reject_deletes_a_pending_memory](../../../../functions/src/tui/app/reject_deletes_a_pending_memory.md)
- [forget_asks_first_and_deletes_only_on_y](../../../../functions/src/tui/app/forget_asks_first_and_deletes_only_on_y.md)
- [enter_shows_details_with_the_history](../../../../functions/src/tui/app/enter_shows_details_with_the_history.md)