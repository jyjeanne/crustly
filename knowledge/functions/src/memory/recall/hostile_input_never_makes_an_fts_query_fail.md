---
type: Rust Function
title: hostile_input_never_makes_an_fts_query_fail
resource: src/memory/recall.rs#L368-L396
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/recall/fts_match_expr
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn hostile_input_never_makes_an_fts_query_fail()`

# Calls

- [fts_match_expr](../../../../functions/src/memory/recall/fts_match_expr.md)