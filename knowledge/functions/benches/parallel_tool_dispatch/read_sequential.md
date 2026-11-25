---
type: Rust Function
title: read_sequential
resource: benches/parallel_tool_dispatch.rs#L24-L31
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/benches/parallel_tool_dispatch/bench_parallel_dispatch
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn read_sequential(paths: &[std::path::PathBuf]) -> Vec<String>`

# Called by

- [bench_parallel_dispatch](../../../functions/benches/parallel_tool_dispatch/bench_parallel_dispatch.md)