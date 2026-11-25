---
type: Rust Function
title: select
resource: src/memory/recall.rs#L127-L191
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/context/token_count
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/recall/is_pinned
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/recall/passes_floor
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/recall/keyword_score
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/recall/recency
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/recall/score
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/MemoryStore/try_recall
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/recall/unmatched_memories_never_take_budget_when_others_compete
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/recall/pinned_memories_use_at_most_half_the_budget
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/recall/everything_fitting_is_taken_without_ranking
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_minor_fixes_test/the_budget_counts_each_lines_overhead_not_just_its_text
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn select( candidates: &[Ranked], budget_tokens: u32, pinned_min_importance: u8, now: DateTime<Utc>, min_similarity: f64, ) -> Vec<Memory>`

# Calls

- [token_count](../../../../functions/src/llm/agent/context/token_count.md)
- [is_pinned](../../../../functions/src/memory/recall/is_pinned.md)
- [passes_floor](../../../../functions/src/memory/recall/passes_floor.md)
- [keyword_score](../../../../functions/src/memory/recall/keyword_score.md)
- [recency](../../../../functions/src/memory/recall/recency.md)
- [score](../../../../functions/src/memory/recall/score.md)

# Called by

- [try_recall](../../../../functions/src/memory/MemoryStore/try_recall.md)
- [unmatched_memories_never_take_budget_when_others_compete](../../../../functions/src/memory/recall/unmatched_memories_never_take_budget_when_others_compete.md)
- [pinned_memories_use_at_most_half_the_budget](../../../../functions/src/memory/recall/pinned_memories_use_at_most_half_the_budget.md)
- [everything_fitting_is_taken_without_ranking](../../../../functions/src/memory/recall/everything_fitting_is_taken_without_ranking.md)
- [the_budget_counts_each_lines_overhead_not_just_its_text](../../../../functions/tests/memory_minor_fixes_test/the_budget_counts_each_lines_overhead_not_just_its_text.md)