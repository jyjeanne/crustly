---
type: Rust Function
title: make_temp_files
resource: benches/parallel_tool_dispatch.rs#L12-L21
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/project/write
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/benches/parallel_tool_dispatch/bench_parallel_dispatch
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn make_temp_files(dir: &TempDir, n: usize) -> Vec<std::path::PathBuf>`

# Calls

- [write](../../../functions/src/memory/project/write.md)

# Called by

- [bench_parallel_dispatch](../../../functions/benches/parallel_tool_dispatch/bench_parallel_dispatch.md)