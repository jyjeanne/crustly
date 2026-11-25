---
type: Rust Function
title: score
resource: src/memory/recall.rs#L98-L104
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/memory/recall/select
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/recall/score_renormalises_without_similarity
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn score(keyword: f64, importance: u8, recency: f64, similarity: Option<f64>) -> f64`

# Called by

- [select](../../../../functions/src/memory/recall/select.md)
- [score_renormalises_without_similarity](../../../../functions/src/memory/recall/score_renormalises_without_similarity.md)