---
type: Rust Method
title: embed
resource: tests/memory_semantic_test.rs#L59-L72
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/tests/memory_semantic_test/topic_vector
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn embed( &self, _purpose: EmbedPurpose, texts: &[String], ) -> Result<Vec<Vec<f32>>, EmbedError>`

# Calls

- [topic_vector](../../../../../functions/tests/memory_semantic_test/topic_vector.md)