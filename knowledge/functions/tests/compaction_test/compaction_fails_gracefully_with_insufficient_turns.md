---
type: Rust Function
title: compaction_fails_gracefully_with_insufficient_turns
resource: tests/compaction_test.rs#L114-L136
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/db/Database/run_migrations
  - functions/tests/compaction_test/build_context
  - functions/src/config/secrets/SecretString/len
  - functions/src/llm/agent/compaction/compact
---

# Signature

`async fn compaction_fails_gracefully_with_insufficient_turns()`

# Calls

- [run_migrations](../../../functions/src/db/Database/run_migrations.md)
- [build_context](../../../functions/tests/compaction_test/build_context.md)
- [len](../../../functions/src/config/secrets/SecretString/len.md)
- [compact](../../../functions/src/llm/agent/compaction/compact.md)