---
type: Rust Function
title: read_sequential
resource: benches/parallel_tool_dispatch.rs#L24-L31
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/len
  called_by:
  - functions/benches/parallel_tool_dispatch/bench_parallel_dispatch
---

# Signature

`async fn read_sequential(paths: &[std::path::PathBuf]) -> Vec<String>`

# Calls

- [len](../../../functions/src/config/secrets/SecretString/len.md)

# Called by

- [bench_parallel_dispatch](../../../functions/benches/parallel_tool_dispatch/bench_parallel_dispatch.md)