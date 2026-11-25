---
type: Rust Function
title: test_to_gemini_request_maps_system_and_tools
resource: src/llm/provider/gemini.rs#L957-L974
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/gemini/GeminiProvider/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/types/LLMRequest/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/types/LLMRequest/with_system
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/LLMRequest/with_tools
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gemini/GeminiProvider/to_gemini_request
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/Message/user
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_to_gemini_request_maps_system_and_tools()`

# Calls

- [new](../../../../../functions/src/llm/provider/gemini/GeminiProvider/new.md)
- [new](../../../../../functions/src/llm/provider/types/LLMRequest/new.md)
- [with_system](../../../../../functions/src/llm/provider/types/LLMRequest/with_system.md)
- [with_tools](../../../../../functions/src/llm/provider/types/LLMRequest/with_tools.md)
- [to_gemini_request](../../../../../functions/src/llm/provider/gemini/GeminiProvider/to_gemini_request.md)
- [user](../../../../../functions/src/llm/provider/types/Message/user.md)