---
type: Rust Module
title: database
resource: benches/database.rs#L1-L326
generated:
  by: okf-rs/0.2.0
relationships:
  imports:
  - external/criterion-black-box-criterion-group-criterion-main-benchmarkid-criterion
  - external/crustly-db-models-session-database
  - external/tempfile-tempdir
  member_of:
  - packages/crustly
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