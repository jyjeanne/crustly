---
type: Rust Function
title: test_llm_request_new_fields
resource: src/llm/provider/openai.rs#L1330-L1346
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
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
  - target: functions/src/llm/provider/types/Message/user
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_llm_request_new_fields()`

# Calls

- [new](../../../../../functions/src/llm/provider/types/LLMRequest/new.md)
- [with_top_p](../../../../../functions/src/llm/provider/types/LLMRequest/with_top_p.md)
- [with_seed](../../../../../functions/src/llm/provider/types/LLMRequest/with_seed.md)
- [with_stop](../../../../../functions/src/llm/provider/types/LLMRequest/with_stop.md)
- [with_frequency_penalty](../../../../../functions/src/llm/provider/types/LLMRequest/with_frequency_penalty.md)
- [with_presence_penalty](../../../../../functions/src/llm/provider/types/LLMRequest/with_presence_penalty.md)
- [with_response_format](../../../../../functions/src/llm/provider/types/LLMRequest/with_response_format.md)
- [user](../../../../../functions/src/llm/provider/types/Message/user.md)