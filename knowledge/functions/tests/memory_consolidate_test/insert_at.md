---
type: Rust Function
title: insert_at
resource: tests/memory_consolidate_test.rs#L48-L77
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/db/repository/memory/MemoryRepository/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/Database/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/memory/MemoryRepository/insert
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/tests/memory_consolidate_test/an_expired_memory_is_archived_on_apply_and_only_reported_before
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_consolidate_test/old_episodes_are_archived_and_recent_ones_kept
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_consolidate_test/a_year_unused_fact_is_listed_for_review_and_never_archived
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_consolidate_test/three_episodes_become_one_lesson_with_its_sources
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_consolidate_test/episodes_feed_one_lesson_only
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_consolidate_test/episodes_archived_by_the_same_run_do_not_feed_a_lesson
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_consolidate_test/lessons_need_extraction_on
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_consolidate_test/the_merge_keeps_categories_apart
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_consolidate_test/the_merge_keeps_the_users_copy_over_a_newer_extracted_one
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn insert_at( db: &Database, project: &ProjectIdentity, category: MemoryCategory, content: &str, created: DateTime<Utc>, expires: Option<DateTime<Utc>>, ) -> Uuid`

# Calls

- [new](../../../functions/src/db/repository/memory/MemoryRepository/new.md)
- [pool](../../../functions/src/db/Database/pool.md)
- [insert](../../../functions/src/db/repository/memory/MemoryRepository/insert.md)

# Called by

- [an_expired_memory_is_archived_on_apply_and_only_reported_before](../../../functions/tests/memory_consolidate_test/an_expired_memory_is_archived_on_apply_and_only_reported_before.md)
- [old_episodes_are_archived_and_recent_ones_kept](../../../functions/tests/memory_consolidate_test/old_episodes_are_archived_and_recent_ones_kept.md)
- [a_year_unused_fact_is_listed_for_review_and_never_archived](../../../functions/tests/memory_consolidate_test/a_year_unused_fact_is_listed_for_review_and_never_archived.md)
- [three_episodes_become_one_lesson_with_its_sources](../../../functions/tests/memory_consolidate_test/three_episodes_become_one_lesson_with_its_sources.md)
- [episodes_feed_one_lesson_only](../../../functions/tests/memory_consolidate_test/episodes_feed_one_lesson_only.md)
- [episodes_archived_by_the_same_run_do_not_feed_a_lesson](../../../functions/tests/memory_consolidate_test/episodes_archived_by_the_same_run_do_not_feed_a_lesson.md)
- [lessons_need_extraction_on](../../../functions/tests/memory_consolidate_test/lessons_need_extraction_on.md)
- [the_merge_keeps_categories_apart](../../../functions/tests/memory_consolidate_test/the_merge_keeps_categories_apart.md)
- [the_merge_keeps_the_users_copy_over_a_newer_extracted_one](../../../functions/tests/memory_consolidate_test/the_merge_keeps_the_users_copy_over_a_newer_extracted_one.md)