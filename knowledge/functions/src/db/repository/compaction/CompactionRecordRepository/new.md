---
type: Rust Method
title: new
resource: src/db/repository/compaction.rs#L14-L16
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/tests/compaction_test/compaction_writes_one_record_to_db
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn new(pool: SqlitePool) -> Self`

# Called by

- [compaction_writes_one_record_to_db](../../../../../../functions/tests/compaction_test/compaction_writes_one_record_to_db.md)