---
type: Rust Method
title: list_for_session
resource: src/db/repository/compaction.rs#L38-L64
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/db/models/PlanTaskStatus/parse
  called_by:
  - functions/tests/compaction_test/compaction_writes_one_record_to_db
---

# Signature

`pub async fn list_for_session(&self, session_id: Uuid) -> Result<Vec<CompactionRecord>>`

# Calls

- [parse](../../../../../../functions/src/db/models/PlanTaskStatus/parse.md)

# Called by

- [compaction_writes_one_record_to_db](../../../../../../functions/tests/compaction_test/compaction_writes_one_record_to_db.md)