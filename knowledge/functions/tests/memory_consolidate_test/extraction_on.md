---
type: Rust Function
title: extraction_on
resource: tests/memory_consolidate_test.rs#L367-L375
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/config/MemoryConfig/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/tests/memory_consolidate_test/episodes_feed_one_lesson_only
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_consolidate_test/episodes_archived_by_the_same_run_do_not_feed_a_lesson
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn extraction_on() -> MemoryConfig`

# Calls

- [default](../../../functions/src/config/MemoryConfig/default/default.md)

# Called by

- [episodes_feed_one_lesson_only](../../../functions/tests/memory_consolidate_test/episodes_feed_one_lesson_only.md)
- [episodes_archived_by_the_same_run_do_not_feed_a_lesson](../../../functions/tests/memory_consolidate_test/episodes_archived_by_the_same_run_do_not_feed_a_lesson.md)