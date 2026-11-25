---
type: Rust Method
title: supported_models
resource: src/llm/provider/ollama.rs#L635-L649
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/provider/ollama/test_supported_models_includes_ornith
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/ollama/test_supported_models_includes_gemma4
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn supported_models(&self) -> Vec<String>`

# Called by

- [test_supported_models_includes_ornith](../../../../../../../functions/src/llm/provider/ollama/test_supported_models_includes_ornith.md)
- [test_supported_models_includes_gemma4](../../../../../../../functions/src/llm/provider/ollama/test_supported_models_includes_gemma4.md)