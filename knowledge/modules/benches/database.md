---
type: Rust Module
title: database
resource: benches/database.rs#L1-L326
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/criterion-black-box-criterion-group-criterion-main-benchmarkid-criterion
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-db-models-session-database
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tempfile-tempdir
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [setup_test_db](../../functions/benches/database/setup_test_db.md)
- [bench_session_create](../../functions/benches/database/bench_session_create.md)
- [bench_session_get](../../functions/benches/database/bench_session_get.md)
- [bench_session_list](../../functions/benches/database/bench_session_list.md)
- [bench_message_insert](../../functions/benches/database/bench_message_insert.md)
- [bench_message_query](../../functions/benches/database/bench_message_query.md)
- [Message](../../classes/benches/database/Message.md)

# Imports

- `criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion}`
- `crustly::db::{models::Session, Database}`
- `tempfile::TempDir`

# Member of

- [crustly](../../packages/crustly.md)