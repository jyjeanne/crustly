---
type: Rust Function
title: recall_in_window
resource: tests/memory_semantic_test.rs#L132-L149
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/recall
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/tests/memory_semantic_test/recall
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_semantic_test/an_unrelated_memory_below_the_threshold_is_not_recalled
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_semantic_test/a_clear_semantic_match_beats_a_weak_keyword_match_under_a_tight_budget
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn recall_in_window( store: &MemoryStore, project: &ProjectIdentity, message: &str, window: u32, ) -> String`

# Calls

- [recall](../../../functions/src/memory/MemoryStore/recall.md)

# Called by

- [recall](../../../functions/tests/memory_semantic_test/recall.md)
- [an_unrelated_memory_below_the_threshold_is_not_recalled](../../../functions/tests/memory_semantic_test/an_unrelated_memory_below_the_threshold_is_not_recalled.md)
- [a_clear_semantic_match_beats_a_weak_keyword_match_under_a_tight_budget](../../../functions/tests/memory_semantic_test/a_clear_semantic_match_beats_a_weak_keyword_match_under_a_tight_budget.md)