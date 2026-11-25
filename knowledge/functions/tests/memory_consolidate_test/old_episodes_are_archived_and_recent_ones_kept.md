---
type: Rust Function
title: old_episodes_are_archived_and_recent_ones_kept
resource: tests/memory_consolidate_test.rs#L107-L138
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/tests/memory_consolidate_test/insert_at
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/consolidate/MemoryStore/consolidate
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn old_episodes_are_archived_and_recent_ones_kept()`

# Calls

- [insert_at](../../../functions/tests/memory_consolidate_test/insert_at.md)
- [consolidate](../../../functions/src/memory/consolidate/MemoryStore/consolidate.md)