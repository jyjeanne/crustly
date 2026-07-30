---
type: Rust Function
title: test_to_gemini_request_maps_system_and_tools
resource: src/llm/provider/gemini.rs#L951-L968
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/provider/types/LLMRequest/with_system
  - functions/src/llm/provider/types/LLMRequest/with_tools
  - functions/src/llm/provider/gemini/GeminiProvider/to_gemini_request
---

# Signature

`fn test_to_gemini_request_maps_system_and_tools()`

# Calls

- [with_system](../../../../../functions/src/llm/provider/types/LLMRequest/with_system.md)
- [with_tools](../../../../../functions/src/llm/provider/types/LLMRequest/with_tools.md)
- [to_gemini_request](../../../../../functions/src/llm/provider/gemini/GeminiProvider/to_gemini_request.md)