---
type: Rust Function
title: test_new_fields_forwarded_to_openai_request
resource: src/llm/provider/openai.rs#L1267-L1285
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/provider/types/LLMRequest/with_top_p
  - functions/src/llm/provider/types/LLMRequest/with_seed
  - functions/src/llm/provider/types/LLMRequest/with_stop
  - functions/src/llm/provider/types/LLMRequest/with_frequency_penalty
  - functions/src/llm/provider/types/LLMRequest/with_presence_penalty
  - functions/src/llm/provider/types/LLMRequest/with_response_format
  - functions/src/llm/provider/openai/OpenAIProvider/to_openai_request
---

# Signature

`fn test_new_fields_forwarded_to_openai_request()`

# Calls

- [with_top_p](../../../../../functions/src/llm/provider/types/LLMRequest/with_top_p.md)
- [with_seed](../../../../../functions/src/llm/provider/types/LLMRequest/with_seed.md)
- [with_stop](../../../../../functions/src/llm/provider/types/LLMRequest/with_stop.md)
- [with_frequency_penalty](../../../../../functions/src/llm/provider/types/LLMRequest/with_frequency_penalty.md)
- [with_presence_penalty](../../../../../functions/src/llm/provider/types/LLMRequest/with_presence_penalty.md)
- [with_response_format](../../../../../functions/src/llm/provider/types/LLMRequest/with_response_format.md)
- [to_openai_request](../../../../../functions/src/llm/provider/openai/OpenAIProvider/to_openai_request.md)