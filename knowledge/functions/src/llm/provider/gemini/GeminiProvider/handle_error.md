---
type: Rust Method
title: handle_error
resource: src/llm/provider/gemini.rs#L345-L354
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/gemini/build_gemini_error
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/gemini/GeminiProvider/provider/complete
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/gemini/GeminiProvider/provider/stream
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn handle_error(&self, response: reqwest::Response) -> ProviderError`

# Calls

- [build_gemini_error](../../../../../../functions/src/llm/provider/gemini/build_gemini_error.md)

# Called by

- [complete](../../../../../../functions/src/llm/provider/gemini/GeminiProvider/provider/complete.md)
- [stream](../../../../../../functions/src/llm/provider/gemini/GeminiProvider/provider/stream.md)