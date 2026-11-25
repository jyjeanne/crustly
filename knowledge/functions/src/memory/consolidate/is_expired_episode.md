---
type: Rust Function
title: is_expired_episode
resource: src/memory/consolidate.rs#L64-L67
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/memory/consolidate/MemoryStore/consolidate
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/consolidate/episodes_expire_after_the_retention_period
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn is_expired_episode(memory: &Memory, retention_days: i64, now: DateTime<Utc>) -> bool`

# Called by

- [consolidate](../../../../functions/src/memory/consolidate/MemoryStore/consolidate.md)
- [episodes_expire_after_the_retention_period](../../../../functions/src/memory/consolidate/episodes_expire_after_the_retention_period.md)