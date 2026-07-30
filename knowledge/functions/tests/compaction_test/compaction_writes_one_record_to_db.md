---
type: Rust Function
title: compaction_writes_one_record_to_db
resource: tests/compaction_test.rs#L156-L171
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/db/Database/run_migrations
  - functions/tests/compaction_test/build_context
  - functions/src/llm/agent/compaction/compact
  - functions/src/db/repository/compaction/CompactionRecordRepository/list_for_session
---

# Signature

`async fn compaction_writes_one_record_to_db()`

# Calls

- [run_migrations](../../../functions/src/db/Database/run_migrations.md)
- [build_context](../../../functions/tests/compaction_test/build_context.md)
- [compact](../../../functions/src/llm/agent/compaction/compact.md)
- [list_for_session](../../../functions/src/db/repository/compaction/CompactionRecordRepository/list_for_session.md)