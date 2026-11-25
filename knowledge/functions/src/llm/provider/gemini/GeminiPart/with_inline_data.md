---
type: Rust Method
title: with_inline_data
resource: src/llm/provider/gemini.rs#L772-L778
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/gemini/GeminiProvider/to_gemini_request
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn with_inline_data(mut self, mime_type: String, data: String) -> Self`

# Called by

- [to_gemini_request](../../../../../../functions/src/llm/provider/gemini/GeminiProvider/to_gemini_request.md)