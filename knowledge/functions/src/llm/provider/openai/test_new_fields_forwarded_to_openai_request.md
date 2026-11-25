---
type: Rust Function
title: test_new_fields_forwarded_to_openai_request
resource: src/llm/provider/openai.rs#L1349-L1367
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/openai/OpenAIProvider/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/types/LLMRequest/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/types/LLMRequest/with_top_p
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/LLMRequest/with_seed
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/LLMRequest/with_stop
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/LLMRequest/with_frequency_penalty
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/LLMRequest/with_presence_penalty
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/LLMRequest/with_response_format
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/openai/OpenAIProvider/to_openai_request
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/Message/user
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_new_fields_forwarded_to_openai_request()`

# Calls

- [new](../../../../../functions/src/llm/provider/openai/OpenAIProvider/new.md)
- [new](../../../../../functions/src/llm/provider/types/LLMRequest/new.md)
- [with_top_p](../../../../../functions/src/llm/provider/types/LLMRequest/with_top_p.md)
- [with_seed](../../../../../functions/src/llm/provider/types/LLMRequest/with_seed.md)
- [with_stop](../../../../../functions/src/llm/provider/types/LLMRequest/with_stop.md)
- [with_frequency_penalty](../../../../../functions/src/llm/provider/types/LLMRequest/with_frequency_penalty.md)
- [with_presence_penalty](../../../../../functions/src/llm/provider/types/LLMRequest/with_presence_penalty.md)
- [with_response_format](../../../../../functions/src/llm/provider/types/LLMRequest/with_response_format.md)
- [to_openai_request](../../../../../functions/src/llm/provider/openai/OpenAIProvider/to_openai_request.md)
- [user](../../../../../functions/src/llm/provider/types/Message/user.md)