---
type: Rust Function
title: compaction_fails_gracefully_with_insufficient_turns
resource: tests/compaction_test.rs#L117-L139
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
  - target: functions/tests/compaction_test/build_context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/compaction/compact
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/Database/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn compaction_fails_gracefully_with_insufficient_turns()`

# Calls

- [connect_in_memory](../../../functions/src/db/Database/connect_in_memory.md)
- [run_migrations](../../../functions/src/db/Database/run_migrations.md)
- [build_context](../../../functions/tests/compaction_test/build_context.md)
- [compact](../../../functions/src/llm/agent/compaction/compact.md)
- [pool](../../../functions/src/db/Database/pool.md)