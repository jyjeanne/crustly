---
type: Rust Method
title: model_id
resource: src/memory/embed.rs#L194-L196
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/memory/embed/ollama_sends_prefixed_input_and_returns_unit_vectors
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn model_id(&self) -> &str`

# Called by

- [ollama_sends_prefixed_input_and_returns_unit_vectors](../../../../../../functions/src/memory/embed/ollama_sends_prefixed_input_and_returns_unit_vectors.md)