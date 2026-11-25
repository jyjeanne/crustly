---
type: Rust Method
title: supported_models
resource: src/llm/provider/gemini.rs#L672-L687
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/gemini/test_supported_models_include_gemma
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn supported_models(&self) -> Vec<String>`

# Called by

- [test_supported_models_include_gemma](../../../../../../../functions/src/llm/provider/gemini/test_supported_models_include_gemma.md)