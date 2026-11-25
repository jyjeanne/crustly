---
type: Rust Function
title: three_episodes_become_one_lesson_with_its_sources
resource: tests/memory_consolidate_test.rs#L326-L365
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/config/MemoryConfig/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/MemoryStore/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/extraction/MemoryStore/with_extraction
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_consolidate_test/FixedAnswer/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_consolidate_test/insert_at
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/consolidate/MemoryStore/consolidate
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/list
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn three_episodes_become_one_lesson_with_its_sources()`

# Calls

- [default](../../../functions/src/config/MemoryConfig/default/default.md)
- [new](../../../functions/src/memory/MemoryStore/new.md)
- [with_extraction](../../../functions/src/memory/extraction/MemoryStore/with_extraction.md)
- [new](../../../functions/tests/memory_consolidate_test/FixedAnswer/new.md)
- [insert_at](../../../functions/tests/memory_consolidate_test/insert_at.md)
- [consolidate](../../../functions/src/memory/consolidate/MemoryStore/consolidate.md)
- [list](../../../functions/src/memory/MemoryStore/list.md)