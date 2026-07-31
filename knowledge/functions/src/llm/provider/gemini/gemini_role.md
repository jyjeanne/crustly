---
type: Rust Function
title: gemini_role
resource: src/llm/provider/gemini.rs#L399-L404
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/llm/provider/gemini/GeminiProvider/to_gemini_request
---

# Signature

`fn gemini_role(role: &Role) -> &'static str`

# Called by

- [to_gemini_request](../../../../../functions/src/llm/provider/gemini/GeminiProvider/to_gemini_request.md)