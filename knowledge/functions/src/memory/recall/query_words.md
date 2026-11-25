---
type: Rust Function
title: query_words
resource: src/memory/recall.rs#L32-L37
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/memory/recall/fts_match_expr
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn query_words(text: &str) -> Vec<String>`

# Called by

- [fts_match_expr](../../../../functions/src/memory/recall/fts_match_expr.md)