---
type: Rust Function
title: compaction_writes_one_record_to_db
resource: tests/compaction_test.rs#L159-L174
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/db/Database/connect_in_memory
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/Database/run_migrations
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/Database/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/compaction_test/build_context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/compaction/compact
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/compaction/CompactionRecordRepository/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/compaction/CompactionRecordRepository/list_for_session
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn compaction_writes_one_record_to_db()`

# Calls

- [connect_in_memory](../../../functions/src/db/Database/connect_in_memory.md)
- [run_migrations](../../../functions/src/db/Database/run_migrations.md)
- [pool](../../../functions/src/db/Database/pool.md)
- [build_context](../../../functions/tests/compaction_test/build_context.md)
- [compact](../../../functions/src/llm/agent/compaction/compact.md)
- [new](../../../functions/src/db/repository/compaction/CompactionRecordRepository/new.md)
- [list_for_session](../../../functions/src/db/repository/compaction/CompactionRecordRepository/list_for_session.md)