---
type: Rust Function
title: backup_stamp
resource: src/memory/transfer.rs#L332-L334
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/cmd_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/transfer/backup_stamps_sort_by_time
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn backup_stamp(now: DateTime<Utc>) -> String`

# Called by

- [cmd_memory](../../../../functions/src/cli/cmd_memory.md)
- [backup_stamps_sort_by_time](../../../../functions/src/memory/transfer/backup_stamps_sort_by_time.md)