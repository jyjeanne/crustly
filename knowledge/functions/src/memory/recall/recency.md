---
type: Rust Function
title: recency
resource: src/memory/recall.rs#L76-L84
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/memory/recall/select
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/recall/recency_is_one_for_standing_rules_and_decays_for_facts
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn recency(category: MemoryCategory, updated_at: DateTime<Utc>, now: DateTime<Utc>) -> f64`

# Called by

- [select](../../../../functions/src/memory/recall/select.md)
- [recency_is_one_for_standing_rules_and_decays_for_facts](../../../../functions/src/memory/recall/recency_is_one_for_standing_rules_and_decays_for_facts.md)