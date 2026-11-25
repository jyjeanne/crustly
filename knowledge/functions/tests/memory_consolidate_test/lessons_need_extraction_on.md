---
type: Rust Function
title: lessons_need_extraction_on
resource: tests/memory_consolidate_test.rs#L446-L471
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/tests/memory_consolidate_test/FixedAnswer/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/MemoryStore/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/MemoryConfig/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/extraction/MemoryStore/with_extraction
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_consolidate_test/insert_at
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/consolidate/MemoryStore/consolidate
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_consolidate_test/FixedAnswer/calls
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn lessons_need_extraction_on()`

# Calls

- [new](../../../functions/tests/memory_consolidate_test/FixedAnswer/new.md)
- [new](../../../functions/src/memory/MemoryStore/new.md)
- [default](../../../functions/src/config/MemoryConfig/default/default.md)
- [with_extraction](../../../functions/src/memory/extraction/MemoryStore/with_extraction.md)
- [insert_at](../../../functions/tests/memory_consolidate_test/insert_at.md)
- [consolidate](../../../functions/src/memory/consolidate/MemoryStore/consolidate.md)
- [calls](../../../functions/tests/memory_consolidate_test/FixedAnswer/calls.md)