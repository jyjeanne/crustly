---
type: Rust Function
title: ranked
resource: src/memory/recall.rs#L311-L317
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/memory/recall/unmatched_memories_never_take_budget_when_others_compete
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/recall/pinned_memories_use_at_most_half_the_budget
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/recall/everything_fitting_is_taken_without_ranking
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn ranked(memory: Memory, keyword: Option<f64>) -> Ranked`

# Called by

- [unmatched_memories_never_take_budget_when_others_compete](../../../../functions/src/memory/recall/unmatched_memories_never_take_budget_when_others_compete.md)
- [pinned_memories_use_at_most_half_the_budget](../../../../functions/src/memory/recall/pinned_memories_use_at_most_half_the_budget.md)
- [everything_fitting_is_taken_without_ranking](../../../../functions/src/memory/recall/everything_fitting_is_taken_without_ranking.md)