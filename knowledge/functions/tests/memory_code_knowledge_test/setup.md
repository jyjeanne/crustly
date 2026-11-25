---
type: Rust Function
title: setup
resource: tests/memory_code_knowledge_test.rs#L198-L224
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/project/write
    resolved_by: tree-sitter
    confidence: exact
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
  - target: functions/src/memory/anchors/MemoryStore/with_code_knowledge
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/CodeKnowledge/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_code_knowledge_test/Scripted/connector
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/project/id_for_root
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/register_project
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn setup(server: &Scripted, timeout: Duration) -> Setup`

# Calls

- [write](../../../functions/src/memory/project/write.md)
- [connect](../../../functions/src/db/Database/connect.md)
- [run_migrations](../../../functions/src/db/Database/run_migrations.md)
- [new](../../../functions/src/memory/MemoryStore/new.md)
- [pool](../../../functions/src/db/Database/pool.md)
- [default](../../../functions/src/config/MemoryConfig/default/default.md)
- [with_code_knowledge](../../../functions/src/memory/anchors/MemoryStore/with_code_knowledge.md)
- [new](../../../functions/src/memory/code_knowledge/CodeKnowledge/new.md)
- [connector](../../../functions/tests/memory_code_knowledge_test/Scripted/connector.md)
- [id_for_root](../../../functions/src/memory/project/id_for_root.md)
- [register_project](../../../functions/src/memory/MemoryStore/register_project.md)