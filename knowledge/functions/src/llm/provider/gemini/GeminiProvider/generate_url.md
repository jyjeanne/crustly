---
type: Rust Method
title: generate_url
resource: src/llm/provider/gemini.rs#L101-L103
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/provider/gemini/GeminiProvider/provider/complete
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn generate_url(&self, model: &str) -> String`

# Called by

- [complete](../../../../../../functions/src/llm/provider/gemini/GeminiProvider/provider/complete.md)