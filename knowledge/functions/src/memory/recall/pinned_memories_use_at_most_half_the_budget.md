---
type: Rust Function
title: pinned_memories_use_at_most_half_the_budget
resource: src/memory/recall.rs#L454-L488
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
  - target: functions/src/llm/agent/context/token_count
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn pinned_memories_use_at_most_half_the_budget()`

# Calls

- [ranked](../../../../functions/src/memory/recall/ranked.md)
- [select](../../../../functions/src/memory/recall/select.md)
- [token_count](../../../../functions/src/llm/agent/context/token_count.md)