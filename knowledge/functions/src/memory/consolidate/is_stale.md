---
type: Rust Function
title: is_stale
resource: src/memory/consolidate.rs#L54-L61
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/memory/consolidate/MemoryStore/consolidate
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/consolidate/a_year_unused_fact_is_stale_but_a_preference_never_is
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn is_stale(memory: &Memory, now: DateTime<Utc>) -> bool`

# Called by

- [consolidate](../../../../functions/src/memory/consolidate/MemoryStore/consolidate.md)
- [a_year_unused_fact_is_stale_but_a_preference_never_is](../../../../functions/src/memory/consolidate/a_year_unused_fact_is_stale_but_a_preference_never_is.md)