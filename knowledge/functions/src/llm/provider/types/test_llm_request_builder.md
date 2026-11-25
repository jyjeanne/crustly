---
type: Rust Function
title: test_llm_request_builder
resource: src/llm/provider/types.rs#L489-L501
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/types/LLMRequest/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/types/LLMRequest/with_system
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/LLMRequest/with_temperature
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/LLMRequest/with_max_tokens
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/LLMRequest/with_streaming
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/Message/user
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_llm_request_builder()`

# Calls

- [new](../../../../../functions/src/llm/provider/types/LLMRequest/new.md)
- [with_system](../../../../../functions/src/llm/provider/types/LLMRequest/with_system.md)
- [with_temperature](../../../../../functions/src/llm/provider/types/LLMRequest/with_temperature.md)
- [with_max_tokens](../../../../../functions/src/llm/provider/types/LLMRequest/with_max_tokens.md)
- [with_streaming](../../../../../functions/src/llm/provider/types/LLMRequest/with_streaming.md)
- [user](../../../../../functions/src/llm/provider/types/Message/user.md)