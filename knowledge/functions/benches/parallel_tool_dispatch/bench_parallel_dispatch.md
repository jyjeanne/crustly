---
type: Rust Function
title: bench_parallel_dispatch
resource: benches/parallel_tool_dispatch.rs#L43-L61
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/benches/parallel_tool_dispatch/make_temp_files
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/benches/parallel_tool_dispatch/read_sequential
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/benches/parallel_tool_dispatch/read_parallel
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn bench_parallel_dispatch(c: &mut Criterion)`

# Calls

- [make_temp_files](../../../functions/benches/parallel_tool_dispatch/make_temp_files.md)
- [read_sequential](../../../functions/benches/parallel_tool_dispatch/read_sequential.md)
- [read_parallel](../../../functions/benches/parallel_tool_dispatch/read_parallel.md)