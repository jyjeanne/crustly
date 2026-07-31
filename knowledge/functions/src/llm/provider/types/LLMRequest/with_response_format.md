---
type: Rust Method
title: with_response_format
resource: src/llm/provider/types.rs#L241-L244
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/llm/provider/gemini/test_json_mode_sets_response_mime_type
  - functions/src/llm/provider/gemini/test_full_json_schema_sets_response_schema
  - functions/src/llm/provider/ollama/to_ollama_request_maps_thinking_and_response_format
  - functions/src/llm/provider/openai/test_llm_request_new_fields
  - functions/src/llm/provider/openai/test_new_fields_forwarded_to_openai_request
---

# Signature

`pub fn with_response_format(mut self, format: serde_json::Value) -> Self`

# Called by

- [test_json_mode_sets_response_mime_type](../../../../../../functions/src/llm/provider/gemini/test_json_mode_sets_response_mime_type.md)
- [test_full_json_schema_sets_response_schema](../../../../../../functions/src/llm/provider/gemini/test_full_json_schema_sets_response_schema.md)
- [to_ollama_request_maps_thinking_and_response_format](../../../../../../functions/src/llm/provider/ollama/to_ollama_request_maps_thinking_and_response_format.md)
- [test_llm_request_new_fields](../../../../../../functions/src/llm/provider/openai/test_llm_request_new_fields.md)
- [test_new_fields_forwarded_to_openai_request](../../../../../../functions/src/llm/provider/openai/test_new_fields_forwarded_to_openai_request.md)