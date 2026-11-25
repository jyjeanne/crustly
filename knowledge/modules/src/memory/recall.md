---
type: Rust Module
title: recall
resource: src/memory/recall.rs#L1-L591
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/super-model-memory-memorycategory-memoryscope
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-llm-agent-context-token-count
    resolved_by: tree-sitter
    confidence: exact
  - target: external/chrono-datetime-utc
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-memory-model-memorysource-memorystatus
    resolved_by: tree-sitter
    confidence: exact
  - target: external/chrono-duration
    resolved_by: tree-sitter
    confidence: exact
  - target: external/uuid-uuid
    resolved_by: tree-sitter
    confidence: exact
  - target: external/sqlx-row
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [query_words](../../../functions/src/memory/recall/query_words.md)
- [fts_match_expr](../../../functions/src/memory/recall/fts_match_expr.md)
- [Ranked](../../../classes/src/memory/recall/Ranked.md)
- [recency](../../../functions/src/memory/recall/recency.md)
- [keyword_score](../../../functions/src/memory/recall/keyword_score.md)
- [score](../../../functions/src/memory/recall/score.md)
- [passes_floor](../../../functions/src/memory/recall/passes_floor.md)
- [is_pinned](../../../functions/src/memory/recall/is_pinned.md)
- [select](../../../functions/src/memory/recall/select.md)
- [short_id](../../../functions/src/memory/recall/short_id.md)
- [clean](../../../functions/src/memory/recall/clean.md)
- [line](../../../functions/src/memory/recall/line.md)
- [render_block](../../../functions/src/memory/recall/render_block.md)
- [now](../../../functions/src/memory/recall/now.md)
- [memory](../../../functions/src/memory/recall/memory.md)
- [ranked](../../../functions/src/memory/recall/ranked.md)
- [query_drops_short_words_and_punctuation](../../../functions/src/memory/recall/query_drops_short_words_and_punctuation.md)
- [query_is_none_without_usable_words](../../../functions/src/memory/recall/query_is_none_without_usable_words.md)
- [short_requests_also_use_the_previous_message](../../../functions/src/memory/recall/short_requests_also_use_the_previous_message.md)
- [query_is_capped_at_32_distinct_words](../../../functions/src/memory/recall/query_is_capped_at_32_distinct_words.md)
- [hostile_input_never_makes_an_fts_query_fail](../../../functions/src/memory/recall/hostile_input_never_makes_an_fts_query_fail.md)
- [everything_fitting_is_taken_without_ranking](../../../functions/src/memory/recall/everything_fitting_is_taken_without_ranking.md)
- [unmatched_memories_never_take_budget_when_others_compete](../../../functions/src/memory/recall/unmatched_memories_never_take_budget_when_others_compete.md)
- [pinned_memories_use_at_most_half_the_budget](../../../functions/src/memory/recall/pinned_memories_use_at_most_half_the_budget.md)
- [recency_is_one_for_standing_rules_and_decays_for_facts](../../../functions/src/memory/recall/recency_is_one_for_standing_rules_and_decays_for_facts.md)
- [keyword_score_is_one_for_the_best_match](../../../functions/src/memory/recall/keyword_score_is_one_for_the_best_match.md)
- [score_renormalises_without_similarity](../../../functions/src/memory/recall/score_renormalises_without_similarity.md)
- [relevance_floor_needs_a_match](../../../functions/src/memory/recall/relevance_floor_needs_a_match.md)
- [no_block_when_nothing_is_selected](../../../functions/src/memory/recall/no_block_when_nothing_is_selected.md)
- [block_has_sections_in_order_and_skips_empty_ones](../../../functions/src/memory/recall/block_has_sections_in_order_and_skips_empty_ones.md)
- [empty_sections_are_left_out](../../../functions/src/memory/recall/empty_sections_are_left_out.md)
- [a_memory_cannot_close_the_block_early](../../../functions/src/memory/recall/a_memory_cannot_close_the_block_early.md)
- [the_block_states_that_memories_are_data](../../../functions/src/memory/recall/the_block_states_that_memories_are_data.md)

# Imports

- `super::model::{Memory, MemoryCategory, MemoryScope}`
- `crate::llm::agent::context::token_count`
- `chrono::{DateTime, Utc}`
- `super::*`
- `crate::memory::model::{MemorySource, MemoryStatus}`
- `chrono::Duration`
- `uuid::Uuid`
- `sqlx::Row`

# Member of

- [crustly](../../../packages/crustly.md)