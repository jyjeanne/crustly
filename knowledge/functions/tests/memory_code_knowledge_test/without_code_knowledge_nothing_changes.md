---
type: Rust Function
title: without_code_knowledge_nothing_changes
resource: tests/memory_code_knowledge_test.rs#L449-L458
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/Database/connect
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
  - target: functions/src/memory/anchors/MemoryStore/code_knowledge_status
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/anchors/MemoryStore/start_code_knowledge
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn without_code_knowledge_nothing_changes()`

# Calls

- [connect](../../../functions/src/db/Database/connect.md)
- [run_migrations](../../../functions/src/db/Database/run_migrations.md)
- [new](../../../functions/src/memory/MemoryStore/new.md)
- [pool](../../../functions/src/db/Database/pool.md)
- [default](../../../functions/src/config/MemoryConfig/default/default.md)
- [code_knowledge_status](../../../functions/src/memory/anchors/MemoryStore/code_knowledge_status.md)
- [start_code_knowledge](../../../functions/src/memory/anchors/MemoryStore/start_code_knowledge.md)