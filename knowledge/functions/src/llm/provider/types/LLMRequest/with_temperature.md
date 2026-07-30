---
type: Rust Method
title: with_temperature
resource: src/llm/provider/types.rs#L203-L206
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/llm/provider/ollama/test_to_ollama_request_maps_common_fields
  - functions/src/llm/provider/types/test_llm_request_builder
---

# Signature

`pub fn with_temperature(mut self, temperature: f32) -> Self`

# Called by

- [test_to_ollama_request_maps_common_fields](../../../../../../functions/src/llm/provider/ollama/test_to_ollama_request_maps_common_fields.md)
- [test_llm_request_builder](../../../../../../functions/src/llm/provider/types/test_llm_request_builder.md)