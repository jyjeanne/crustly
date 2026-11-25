---
type: Rust Function
title: gemini_role
resource: src/llm/provider/gemini.rs#L405-L410
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/gemini/GeminiProvider/to_gemini_request
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gemini/test_role_mapping
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn gemini_role(role: &Role) -> &'static str`

# Called by

- [to_gemini_request](../../../../../functions/src/llm/provider/gemini/GeminiProvider/to_gemini_request.md)
- [test_role_mapping](../../../../../functions/src/llm/provider/gemini/test_role_mapping.md)