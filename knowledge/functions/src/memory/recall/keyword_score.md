---
type: Rust Function
title: keyword_score
resource: src/memory/recall.rs#L88-L94
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/memory/recall/select
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/recall/keyword_score_is_one_for_the_best_match
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn keyword_score(bm25: Option<f64>, best_bm25: f64) -> f64`

# Called by

- [select](../../../../functions/src/memory/recall/select.md)
- [keyword_score_is_one_for_the_best_match](../../../../functions/src/memory/recall/keyword_score_is_one_for_the_best_match.md)