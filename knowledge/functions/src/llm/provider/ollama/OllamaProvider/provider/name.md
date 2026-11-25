---
type: Rust Method
title: name
resource: src/llm/provider/ollama.rs#L627-L629
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/provider/ollama/test_ollama_provider_creation
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/ollama/test_invalid_host_falls_back_to_default
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn name(&self) -> &str`

# Called by

- [test_ollama_provider_creation](../../../../../../../functions/src/llm/provider/ollama/test_ollama_provider_creation.md)
- [test_invalid_host_falls_back_to_default](../../../../../../../functions/src/llm/provider/ollama/test_invalid_host_falls_back_to_default.md)