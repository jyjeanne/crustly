---
type: Rust Function
title: fast_runtime
resource: tests/memory_semantic_test.rs#L75-L82
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/tests/memory_semantic_test/a_memory_is_recalled_by_meaning_without_a_shared_word
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_semantic_test/an_unrelated_memory_below_the_threshold_is_not_recalled
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_semantic_test/when_the_embedder_is_down_keyword_recall_still_works
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_semantic_test/a_failed_embedding_is_retried_and_then_stored
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_semantic_test/reindex_embeds_memories_saved_before_the_embedder_was_configured
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_semantic_test/two_memories_saying_the_same_thing_are_recalled_once
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_semantic_test/one_of_two_near_duplicates_is_archived_not_deleted
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_semantic_test/an_edited_memory_is_embedded_again_from_its_new_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_semantic_test/an_approved_memory_is_embedded
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_semantic_test/a_clear_semantic_match_beats_a_weak_keyword_match_under_a_tight_budget
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn fast_runtime() -> EmbedRuntime`

# Called by

- [a_memory_is_recalled_by_meaning_without_a_shared_word](../../../functions/tests/memory_semantic_test/a_memory_is_recalled_by_meaning_without_a_shared_word.md)
- [an_unrelated_memory_below_the_threshold_is_not_recalled](../../../functions/tests/memory_semantic_test/an_unrelated_memory_below_the_threshold_is_not_recalled.md)
- [when_the_embedder_is_down_keyword_recall_still_works](../../../functions/tests/memory_semantic_test/when_the_embedder_is_down_keyword_recall_still_works.md)
- [a_failed_embedding_is_retried_and_then_stored](../../../functions/tests/memory_semantic_test/a_failed_embedding_is_retried_and_then_stored.md)
- [reindex_embeds_memories_saved_before_the_embedder_was_configured](../../../functions/tests/memory_semantic_test/reindex_embeds_memories_saved_before_the_embedder_was_configured.md)
- [two_memories_saying_the_same_thing_are_recalled_once](../../../functions/tests/memory_semantic_test/two_memories_saying_the_same_thing_are_recalled_once.md)
- [one_of_two_near_duplicates_is_archived_not_deleted](../../../functions/tests/memory_semantic_test/one_of_two_near_duplicates_is_archived_not_deleted.md)
- [an_edited_memory_is_embedded_again_from_its_new_text](../../../functions/tests/memory_semantic_test/an_edited_memory_is_embedded_again_from_its_new_text.md)
- [an_approved_memory_is_embedded](../../../functions/tests/memory_semantic_test/an_approved_memory_is_embedded.md)
- [a_clear_semantic_match_beats_a_weak_keyword_match_under_a_tight_budget](../../../functions/tests/memory_semantic_test/a_clear_semantic_match_beats_a_weak_keyword_match_under_a_tight_budget.md)