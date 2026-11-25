---
type: Rust Function
title: the_budget_counts_each_lines_overhead_not_just_its_text
resource: tests/memory_minor_fixes_test.rs#L124-L148
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/context/token_count
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/recall/select
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn the_budget_counts_each_lines_overhead_not_just_its_text()`

# Calls

- [token_count](../../../functions/src/llm/agent/context/token_count.md)
- [select](../../../functions/src/memory/recall/select.md)