---
type: Rust Function
title: bench_parallel_dispatch
resource: benches/parallel_tool_dispatch.rs#L43-L61
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/benches/parallel_tool_dispatch/make_temp_files
  - functions/benches/parallel_tool_dispatch/read_sequential
  - functions/benches/parallel_tool_dispatch/read_parallel
  - functions/src/tui/markdown/MarkdownRenderer/finish
---

# Signature

`fn bench_parallel_dispatch(c: &mut Criterion)`

# Calls

- [make_temp_files](../../../functions/benches/parallel_tool_dispatch/make_temp_files.md)
- [read_sequential](../../../functions/benches/parallel_tool_dispatch/read_sequential.md)
- [read_parallel](../../../functions/benches/parallel_tool_dispatch/read_parallel.md)
- [finish](../../../functions/src/tui/markdown/MarkdownRenderer/finish.md)