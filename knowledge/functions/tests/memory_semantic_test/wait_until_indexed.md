---
type: Rust Function
title: wait_until_indexed
resource: tests/memory_semantic_test.rs#L116-L126
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/embedding_status
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/tests/memory_semantic_test/a_memory_is_recalled_by_meaning_without_a_shared_word
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_semantic_test/an_unrelated_memory_below_the_threshold_is_not_recalled
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
  - target: functions/tests/memory_semantic_test/a_clear_semantic_match_beats_a_weak_keyword_match_under_a_tight_budget
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn wait_until_indexed(store: &MemoryStore, count: i64)`

# Calls

- [embedding_status](../../../functions/src/memory/MemoryStore/embedding_status.md)

# Called by

- [a_memory_is_recalled_by_meaning_without_a_shared_word](../../../functions/tests/memory_semantic_test/a_memory_is_recalled_by_meaning_without_a_shared_word.md)
- [an_unrelated_memory_below_the_threshold_is_not_recalled](../../../functions/tests/memory_semantic_test/an_unrelated_memory_below_the_threshold_is_not_recalled.md)
- [a_failed_embedding_is_retried_and_then_stored](../../../functions/tests/memory_semantic_test/a_failed_embedding_is_retried_and_then_stored.md)
- [reindex_embeds_memories_saved_before_the_embedder_was_configured](../../../functions/tests/memory_semantic_test/reindex_embeds_memories_saved_before_the_embedder_was_configured.md)
- [two_memories_saying_the_same_thing_are_recalled_once](../../../functions/tests/memory_semantic_test/two_memories_saying_the_same_thing_are_recalled_once.md)
- [a_clear_semantic_match_beats_a_weak_keyword_match_under_a_tight_budget](../../../functions/tests/memory_semantic_test/a_clear_semantic_match_beats_a_weak_keyword_match_under_a_tight_budget.md)