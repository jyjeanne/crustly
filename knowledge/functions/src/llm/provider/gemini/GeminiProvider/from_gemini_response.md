---
type: Rust Method
title: from_gemini_response
resource: src/llm/provider/gemini.rs#L265-L337
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/tui/events/EventHandler/next
  - functions/src/config/secrets/SecretString/is_empty
  called_by:
  - functions/src/llm/provider/gemini/GeminiProvider/provider/complete
  - functions/src/llm/provider/gemini/test_from_gemini_response_maps_tool_use
  - functions/src/llm/provider/gemini/test_from_gemini_response_maps_thinking
---

# Signature

`fn from_gemini_response(&self, response: GeminiResponse, model: &str) -> LLMResponse`

# Calls

- [next](../../../../../../functions/src/tui/events/EventHandler/next.md)
- [is_empty](../../../../../../functions/src/config/secrets/SecretString/is_empty.md)

# Called by

- [complete](../../../../../../functions/src/llm/provider/gemini/GeminiProvider/provider/complete.md)
- [test_from_gemini_response_maps_tool_use](../../../../../../functions/src/llm/provider/gemini/test_from_gemini_response_maps_tool_use.md)
- [test_from_gemini_response_maps_thinking](../../../../../../functions/src/llm/provider/gemini/test_from_gemini_response_maps_thinking.md)