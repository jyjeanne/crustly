---
type: Rust Method
title: consolidate
resource: src/memory/consolidate.rs#L72-L168
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/repository/memory/MemoryRepository/list_for_project
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/archive
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/consolidate/is_expired_episode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/consolidate/is_stale
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/consolidate/MemoryStore/embedding_model
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/vectors_for
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/PlanTask/skip
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/consolidate/pair_similarity
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/semantic/near_duplicate_to_archive
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/tool_round/ToolRound/push
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/supersede
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/consolidate/MemoryStore/lesson_from_episodes
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/cmd_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_consolidate_test/an_expired_memory_is_archived_on_apply_and_only_reported_before
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_consolidate_test/old_episodes_are_archived_and_recent_ones_kept
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_consolidate_test/a_year_unused_fact_is_listed_for_review_and_never_archived
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_consolidate_test/near_duplicates_in_the_merge_band_are_merged_on_apply
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_consolidate_test/three_episodes_become_one_lesson_with_its_sources
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_consolidate_test/episodes_archived_by_the_same_run_do_not_feed_a_lesson
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_consolidate_test/the_merge_keeps_categories_apart
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_consolidate_test/the_merge_keeps_the_users_copy_over_a_newer_extracted_one
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_consolidate_test/episodes_feed_one_lesson_only
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_consolidate_test/lessons_need_extraction_on
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn consolidate( &self, project: &ProjectIdentity, apply: bool, ) -> Result<ConsolidationReport>`

# Calls

- [list_for_project](../../../../../functions/src/db/repository/memory/MemoryRepository/list_for_project.md)
- [archive](../../../../../functions/src/db/repository/memory/MemoryRepository/archive.md)
- [is_expired_episode](../../../../../functions/src/memory/consolidate/is_expired_episode.md)
- [is_stale](../../../../../functions/src/memory/consolidate/is_stale.md)
- [embedding_model](../../../../../functions/src/memory/consolidate/MemoryStore/embedding_model.md)
- [vectors_for](../../../../../functions/src/db/repository/memory/MemoryRepository/vectors_for.md)
- [skip](../../../../../functions/src/plan/PlanTask/skip.md)
- [pair_similarity](../../../../../functions/src/memory/consolidate/pair_similarity.md)
- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)
- [near_duplicate_to_archive](../../../../../functions/src/memory/semantic/near_duplicate_to_archive.md)
- [push](../../../../../functions/src/llm/agent/service/tool_round/ToolRound/push.md)
- [supersede](../../../../../functions/src/db/repository/memory/MemoryRepository/supersede.md)
- [lesson_from_episodes](../../../../../functions/src/memory/consolidate/MemoryStore/lesson_from_episodes.md)

# Called by

- [cmd_memory](../../../../../functions/src/cli/cmd_memory.md)
- [an_expired_memory_is_archived_on_apply_and_only_reported_before](../../../../../functions/tests/memory_consolidate_test/an_expired_memory_is_archived_on_apply_and_only_reported_before.md)
- [old_episodes_are_archived_and_recent_ones_kept](../../../../../functions/tests/memory_consolidate_test/old_episodes_are_archived_and_recent_ones_kept.md)
- [a_year_unused_fact_is_listed_for_review_and_never_archived](../../../../../functions/tests/memory_consolidate_test/a_year_unused_fact_is_listed_for_review_and_never_archived.md)
- [near_duplicates_in_the_merge_band_are_merged_on_apply](../../../../../functions/tests/memory_consolidate_test/near_duplicates_in_the_merge_band_are_merged_on_apply.md)
- [three_episodes_become_one_lesson_with_its_sources](../../../../../functions/tests/memory_consolidate_test/three_episodes_become_one_lesson_with_its_sources.md)
- [episodes_archived_by_the_same_run_do_not_feed_a_lesson](../../../../../functions/tests/memory_consolidate_test/episodes_archived_by_the_same_run_do_not_feed_a_lesson.md)
- [the_merge_keeps_categories_apart](../../../../../functions/tests/memory_consolidate_test/the_merge_keeps_categories_apart.md)
- [the_merge_keeps_the_users_copy_over_a_newer_extracted_one](../../../../../functions/tests/memory_consolidate_test/the_merge_keeps_the_users_copy_over_a_newer_extracted_one.md)
- [episodes_feed_one_lesson_only](../../../../../functions/tests/memory_consolidate_test/episodes_feed_one_lesson_only.md)
- [lessons_need_extraction_on](../../../../../functions/tests/memory_consolidate_test/lessons_need_extraction_on.md)