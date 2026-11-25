---
type: Rust Function
title: a_database_from_before_memory_upgrades_and_keeps_its_sessions
resource: tests/memory_migration_test.rs#L17-L71
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/db/Database/close
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/Database/connect
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/Database/run_migrations
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/Database/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/skill/SkillSource/display
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn a_database_from_before_memory_upgrades_and_keeps_its_sessions()`

# Calls

- [close](../../../functions/src/db/Database/close.md)
- [connect](../../../functions/src/db/Database/connect.md)
- [run_migrations](../../../functions/src/db/Database/run_migrations.md)
- [pool](../../../functions/src/db/Database/pool.md)
- [display](../../../functions/src/llm/tools/skill/SkillSource/display.md)