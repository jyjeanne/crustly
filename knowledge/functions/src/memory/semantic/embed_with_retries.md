---
type: Rust Function
title: embed_with_retries
resource: src/memory/semantic.rs#L72-L92
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/memory/semantic/embed_and_store
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn embed_with_retries( embedder: &dyn Embedder, purpose: EmbedPurpose, texts: &[String], retry_delays: &[Duration], ) -> Result<Vec<Vec<f32>>, EmbedError>`

# Called by

- [embed_and_store](../../../../functions/src/memory/semantic/embed_and_store.md)