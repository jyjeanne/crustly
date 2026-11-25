---
type: Rust Function
title: report
resource: tests/memory_benchmark_test.rs#L249-L254
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/tests/memory_benchmark_test/the_harness_runs_and_meets_the_ci_floor
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_benchmark_test/real_embedder_benchmark
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn report(label: &str, m: &Metrics)`

# Called by

- [the_harness_runs_and_meets_the_ci_floor](../../../functions/tests/memory_benchmark_test/the_harness_runs_and_meets_the_ci_floor.md)
- [real_embedder_benchmark](../../../functions/tests/memory_benchmark_test/real_embedder_benchmark.md)