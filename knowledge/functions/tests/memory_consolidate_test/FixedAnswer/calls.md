---
type: Rust Method
title: calls
resource: tests/memory_consolidate_test.rs#L275-L277
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/tests/memory_consolidate_test/episodes_feed_one_lesson_only
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_consolidate_test/episodes_archived_by_the_same_run_do_not_feed_a_lesson
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_consolidate_test/lessons_need_extraction_on
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn calls(&self) -> usize`

# Called by

- [episodes_feed_one_lesson_only](../../../../functions/tests/memory_consolidate_test/episodes_feed_one_lesson_only.md)
- [episodes_archived_by_the_same_run_do_not_feed_a_lesson](../../../../functions/tests/memory_consolidate_test/episodes_archived_by_the_same_run_do_not_feed_a_lesson.md)
- [lessons_need_extraction_on](../../../../functions/tests/memory_consolidate_test/lessons_need_extraction_on.md)