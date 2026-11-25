---
type: Rust Function
title: passes_floor
resource: src/memory/recall.rs#L108-L110
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/memory/recall/select
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/recall/relevance_floor_needs_a_match
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn passes_floor(keyword: Option<f64>, similarity: Option<f64>, min_similarity: f64) -> bool`

# Called by

- [select](../../../../functions/src/memory/recall/select.md)
- [relevance_floor_needs_a_match](../../../../functions/src/memory/recall/relevance_floor_needs_a_match.md)