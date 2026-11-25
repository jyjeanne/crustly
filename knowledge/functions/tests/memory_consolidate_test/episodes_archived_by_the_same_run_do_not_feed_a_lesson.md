---
type: Rust Function
title: episodes_archived_by_the_same_run_do_not_feed_a_lesson
resource: tests/memory_consolidate_test.rs#L421-L443
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/tests/memory_consolidate_test/FixedAnswer/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/MemoryStore/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_consolidate_test/extraction_on
    resolved_by: tree-sitter
    confidence: exact
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

`async fn episodes_archived_by_the_same_run_do_not_feed_a_lesson()`

# Calls

- [new](../../../functions/tests/memory_consolidate_test/FixedAnswer/new.md)
- [new](../../../functions/src/memory/MemoryStore/new.md)
- [extraction_on](../../../functions/tests/memory_consolidate_test/extraction_on.md)
- [with_extraction](../../../functions/src/memory/extraction/MemoryStore/with_extraction.md)
- [insert_at](../../../functions/tests/memory_consolidate_test/insert_at.md)
- [consolidate](../../../functions/src/memory/consolidate/MemoryStore/consolidate.md)
- [calls](../../../functions/tests/memory_consolidate_test/FixedAnswer/calls.md)