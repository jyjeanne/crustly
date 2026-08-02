---
type: Rust Method
title: text
resource: src/llm/provider/gemini.rs#L759-L764
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/llm/provider/gemini/GeminiProvider/to_gemini_request
  - functions/src/llm/tools/http/HttpClientTool/tool/execute
---

# Signature

`fn text(text: String) -> Self`

# Called by

- [to_gemini_request](../../../../../../functions/src/llm/provider/gemini/GeminiProvider/to_gemini_request.md)
- [execute](../../../../../../functions/src/llm/tools/http/HttpClientTool/tool/execute.md)