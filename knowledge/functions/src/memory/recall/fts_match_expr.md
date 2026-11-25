---
type: Rust Function
title: fts_match_expr
resource: src/memory/recall.rs#L42-L62
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/recall/query_words
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/MemoryStore/try_recall
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/search
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/recall/short_requests_also_use_the_previous_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/recall/query_is_capped_at_32_distinct_words
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/recall/hostile_input_never_makes_an_fts_query_fail
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/recall/query_drops_short_words_and_punctuation
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/recall/query_is_none_without_usable_words
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn fts_match_expr(message: &str, previous: Option<&str>) -> Option<String>`

# Calls

- [query_words](../../../../functions/src/memory/recall/query_words.md)

# Called by

- [try_recall](../../../../functions/src/memory/MemoryStore/try_recall.md)
- [search](../../../../functions/src/memory/MemoryStore/search.md)
- [short_requests_also_use_the_previous_message](../../../../functions/src/memory/recall/short_requests_also_use_the_previous_message.md)
- [query_is_capped_at_32_distinct_words](../../../../functions/src/memory/recall/query_is_capped_at_32_distinct_words.md)
- [hostile_input_never_makes_an_fts_query_fail](../../../../functions/src/memory/recall/hostile_input_never_makes_an_fts_query_fail.md)
- [query_drops_short_words_and_punctuation](../../../../functions/src/memory/recall/query_drops_short_words_and_punctuation.md)
- [query_is_none_without_usable_words](../../../../functions/src/memory/recall/query_is_none_without_usable_words.md)