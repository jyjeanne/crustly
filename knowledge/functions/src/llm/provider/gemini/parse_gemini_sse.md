---
type: Rust Function
title: parse_gemini_sse
resource: src/llm/provider/gemini.rs#L412-L538
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/is_empty
  - functions/src/tui/events/EventHandler/next
  called_by:
  - functions/src/llm/provider/gemini/GeminiProvider/provider/stream
  - functions/src/llm/provider/gemini/test_parse_gemini_sse_text_response
  - functions/src/llm/provider/gemini/test_parse_gemini_sse_thinking_part
  - functions/src/llm/provider/gemini/test_parse_gemini_sse_function_call
  - functions/src/llm/provider/gemini/test_parse_gemini_sse_max_tokens
  - functions/src/llm/provider/gemini/test_parse_gemini_sse_skips_malformed_lines
  - functions/src/llm/provider/gemini/test_parse_gemini_sse_ignores_non_data_lines
---

# Signature

`fn parse_gemini_sse(text: &str, model: &str) -> Vec<StreamEvent>`

# Calls

- [is_empty](../../../../../functions/src/config/secrets/SecretString/is_empty.md)
- [next](../../../../../functions/src/tui/events/EventHandler/next.md)

# Called by

- [stream](../../../../../functions/src/llm/provider/gemini/GeminiProvider/provider/stream.md)
- [test_parse_gemini_sse_text_response](../../../../../functions/src/llm/provider/gemini/test_parse_gemini_sse_text_response.md)
- [test_parse_gemini_sse_thinking_part](../../../../../functions/src/llm/provider/gemini/test_parse_gemini_sse_thinking_part.md)
- [test_parse_gemini_sse_function_call](../../../../../functions/src/llm/provider/gemini/test_parse_gemini_sse_function_call.md)
- [test_parse_gemini_sse_max_tokens](../../../../../functions/src/llm/provider/gemini/test_parse_gemini_sse_max_tokens.md)
- [test_parse_gemini_sse_skips_malformed_lines](../../../../../functions/src/llm/provider/gemini/test_parse_gemini_sse_skips_malformed_lines.md)
- [test_parse_gemini_sse_ignores_non_data_lines](../../../../../functions/src/llm/provider/gemini/test_parse_gemini_sse_ignores_non_data_lines.md)