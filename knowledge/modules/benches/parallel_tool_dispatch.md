---
type: Rust Module
title: parallel_tool_dispatch
resource: benches/parallel_tool_dispatch.rs#L1-L64
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/criterion-criterion-group-criterion-main-criterion
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-time-duration
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tempfile-tempdir
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tokio-runtime-runtime
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [make_temp_files](../../functions/benches/parallel_tool_dispatch/make_temp_files.md)
- [read_sequential](../../functions/benches/parallel_tool_dispatch/read_sequential.md)
- [read_parallel](../../functions/benches/parallel_tool_dispatch/read_parallel.md)
- [bench_parallel_dispatch](../../functions/benches/parallel_tool_dispatch/bench_parallel_dispatch.md)

# Imports

- `criterion::{criterion_group, criterion_main, Criterion}`
- `std::time::Duration`
- `tempfile::TempDir`
- `tokio::runtime::Runtime`

# Member of

- [crustly](../../packages/crustly.md)