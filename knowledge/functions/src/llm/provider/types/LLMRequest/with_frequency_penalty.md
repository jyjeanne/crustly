---
type: Rust Method
title: with_frequency_penalty
resource: src/llm/provider/types.rs#L228-L231
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/llm/provider/openai/test_llm_request_new_fields
  - functions/src/llm/provider/openai/test_new_fields_forwarded_to_openai_request
---

# Signature

`pub fn with_frequency_penalty(mut self, penalty: f32) -> Self`

# Called by

- [test_llm_request_new_fields](../../../../../../functions/src/llm/provider/openai/test_llm_request_new_fields.md)
- [test_new_fields_forwarded_to_openai_request](../../../../../../functions/src/llm/provider/openai/test_new_fields_forwarded_to_openai_request.md)