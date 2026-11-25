---
type: Rust Function
title: read_parallel
resource: benches/parallel_tool_dispatch.rs#L34-L41
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

`async fn read_parallel(paths: &[std::path::PathBuf]) -> Vec<String>`

# Called by

- [bench_parallel_dispatch](../../../functions/benches/parallel_tool_dispatch/bench_parallel_dispatch.md)