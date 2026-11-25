---
type: Rust Function
title: everything_fitting_is_taken_without_ranking
resource: src/memory/recall.rs#L401-L413
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/recall/ranked
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/recall/select
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn everything_fitting_is_taken_without_ranking()`

# Calls

- [ranked](../../../../functions/src/memory/recall/ranked.md)
- [select](../../../../functions/src/memory/recall/select.md)