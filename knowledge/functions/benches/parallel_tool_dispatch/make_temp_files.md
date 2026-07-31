---
type: Rust Function
title: make_temp_files
resource: benches/parallel_tool_dispatch.rs#L12-L21
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/benches/parallel_tool_dispatch/bench_parallel_dispatch
---

# Signature

`fn make_temp_files(dir: &TempDir, n: usize) -> Vec<std::path::PathBuf>`

# Called by

- [bench_parallel_dispatch](../../../functions/benches/parallel_tool_dispatch/bench_parallel_dispatch.md)