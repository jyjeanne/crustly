---
type: Rust Function
title: compaction_preserves_last_10_turns
resource: tests/compaction_test.rs#L57-L106
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/db/Database/run_migrations
  - functions/tests/compaction_test/build_context
  - functions/src/llm/agent/compaction/compact
---

# Signature

`async fn compaction_preserves_last_10_turns()`

# Calls

- [run_migrations](../../../functions/src/db/Database/run_migrations.md)
- [build_context](../../../functions/tests/compaction_test/build_context.md)
- [compact](../../../functions/src/llm/agent/compaction/compact.md)