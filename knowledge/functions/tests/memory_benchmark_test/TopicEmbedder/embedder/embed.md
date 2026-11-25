---
type: Rust Method
title: embed
resource: tests/memory_benchmark_test.rs#L73-L111
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn embed( &self, _purpose: EmbedPurpose, texts: &[String], ) -> Result<Vec<Vec<f32>>, EmbedError>`

# Calls

- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)