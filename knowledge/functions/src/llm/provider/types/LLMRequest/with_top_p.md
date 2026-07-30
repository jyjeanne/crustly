---
type: Rust Method
title: with_top_p
resource: src/llm/provider/types.rs#L210-L213
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/llm/provider/ollama/test_to_ollama_request_maps_common_fields
  - functions/src/llm/provider/openai/test_llm_request_new_fields
  - functions/src/llm/provider/openai/test_new_fields_forwarded_to_openai_request
  - functions/src/llm/provider/qwen/test_sampling_explicit_request_top_p_wins
---

# Signature

`pub fn with_top_p(mut self, top_p: f32) -> Self`

# Called by

- [test_to_ollama_request_maps_common_fields](../../../../../../functions/src/llm/provider/ollama/test_to_ollama_request_maps_common_fields.md)
- [test_llm_request_new_fields](../../../../../../functions/src/llm/provider/openai/test_llm_request_new_fields.md)
- [test_new_fields_forwarded_to_openai_request](../../../../../../functions/src/llm/provider/openai/test_new_fields_forwarded_to_openai_request.md)
- [test_sampling_explicit_request_top_p_wins](../../../../../../functions/src/llm/provider/qwen/test_sampling_explicit_request_top_p_wins.md)