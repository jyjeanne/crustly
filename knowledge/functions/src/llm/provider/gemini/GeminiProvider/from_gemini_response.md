---
type: Rust Method
title: from_gemini_response
resource: src/llm/provider/gemini.rs#L271-L343
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/events/EventHandler/next
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/gemini/GeminiProvider/provider/complete
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gemini/test_from_gemini_response_maps_tool_use
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gemini/test_from_gemini_response_maps_thinking
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn from_gemini_response(&self, response: GeminiResponse, model: &str) -> LLMResponse`

# Calls

- [next](../../../../../../functions/src/tui/events/EventHandler/next.md)

# Called by

- [complete](../../../../../../functions/src/llm/provider/gemini/GeminiProvider/provider/complete.md)
- [test_from_gemini_response_maps_tool_use](../../../../../../functions/src/llm/provider/gemini/test_from_gemini_response_maps_tool_use.md)
- [test_from_gemini_response_maps_thinking](../../../../../../functions/src/llm/provider/gemini/test_from_gemini_response_maps_thinking.md)