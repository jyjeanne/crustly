---
type: Rust Method
title: to_openai_request
resource: src/llm/provider/openai.rs#L188-L344
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/is_empty
  called_by:
  - functions/src/llm/provider/openai/OpenAIProvider/provider/complete
  - functions/src/llm/provider/openai/OpenAIProvider/provider/stream
  - functions/src/llm/provider/openai/test_new_fields_forwarded_to_openai_request
---

# Signature

`fn to_openai_request(&self, request: LLMRequest) -> OpenAIRequest`

# Calls

- [is_empty](../../../../../../functions/src/config/secrets/SecretString/is_empty.md)

# Called by

- [complete](../../../../../../functions/src/llm/provider/openai/OpenAIProvider/provider/complete.md)
- [stream](../../../../../../functions/src/llm/provider/openai/OpenAIProvider/provider/stream.md)
- [test_new_fields_forwarded_to_openai_request](../../../../../../functions/src/llm/provider/openai/test_new_fields_forwarded_to_openai_request.md)