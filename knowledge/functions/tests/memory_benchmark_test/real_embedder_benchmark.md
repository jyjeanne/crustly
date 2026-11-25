---
type: Rust Function
title: real_embedder_benchmark
resource: tests/memory_benchmark_test.rs#L286-L310
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/embed/Prefixes/for_model
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/embed/OllamaEmbedder/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/semantic/EmbedRuntime/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_benchmark_test/report
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn real_embedder_benchmark()`

# Calls

- [for_model](../../../functions/src/memory/embed/Prefixes/for_model.md)
- [new](../../../functions/src/memory/embed/OllamaEmbedder/new.md)
- [default](../../../functions/src/memory/semantic/EmbedRuntime/default/default.md)
- [report](../../../functions/tests/memory_benchmark_test/report.md)