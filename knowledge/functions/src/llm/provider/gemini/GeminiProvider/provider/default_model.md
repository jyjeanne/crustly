---
type: Rust Method
title: default_model
resource: src/llm/provider/gemini.rs#L666-L670
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/provider/gemini/test_gemini_provider_creation
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/gemini/test_custom_default_model
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn default_model(&self) -> &str`

# Called by

- [test_gemini_provider_creation](../../../../../../../functions/src/llm/provider/gemini/test_gemini_provider_creation.md)
- [test_custom_default_model](../../../../../../../functions/src/llm/provider/gemini/test_custom_default_model.md)