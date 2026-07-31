---
type: Rust Method
title: with_seed
resource: src/llm/provider/types.rs#L216-L219
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/llm/provider/ollama/test_to_ollama_request_maps_common_fields
  - functions/src/llm/provider/openai/test_llm_request_new_fields
  - functions/src/llm/provider/openai/test_new_fields_forwarded_to_openai_request
---

# Signature

`pub fn with_seed(mut self, seed: u64) -> Self`

# Called by

- [test_to_ollama_request_maps_common_fields](../../../../../../functions/src/llm/provider/ollama/test_to_ollama_request_maps_common_fields.md)
- [test_llm_request_new_fields](../../../../../../functions/src/llm/provider/openai/test_llm_request_new_fields.md)
- [test_new_fields_forwarded_to_openai_request](../../../../../../functions/src/llm/provider/openai/test_new_fields_forwarded_to_openai_request.md)