---
type: Rust Method
title: with_base_url
resource: src/llm/provider/gemini.rs#L66-L70
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/gemini/GeminiProvider/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/llm/provider/factory/try_create_gemini
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn with_base_url(api_key: String, base_url: String) -> Self`

# Calls

- [new](../../../../../../functions/src/llm/provider/gemini/GeminiProvider/new.md)

# Called by

- [try_create_gemini](../../../../../../functions/src/llm/provider/factory/try_create_gemini.md)