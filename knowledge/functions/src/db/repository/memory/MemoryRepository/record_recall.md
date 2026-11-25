---
type: Rust Method
title: record_recall
resource: src/db/repository/memory.rs#L590-L604
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/memory/MemoryStore/try_recall
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn record_recall(&self, ids: &[Uuid], now: DateTime<Utc>) -> Result<()>`

# Called by

- [try_recall](../../../../../../functions/src/memory/MemoryStore/try_recall.md)