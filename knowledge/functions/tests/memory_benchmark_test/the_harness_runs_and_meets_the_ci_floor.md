---
type: Rust Function
title: the_harness_runs_and_meets_the_ci_floor
resource: tests/memory_benchmark_test.rs#L257-L282
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/semantic/EmbedRuntime/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_benchmark_test/report
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn the_harness_runs_and_meets_the_ci_floor()`

# Calls

- [default](../../../functions/src/memory/semantic/EmbedRuntime/default/default.md)
- [report](../../../functions/tests/memory_benchmark_test/report.md)