---
type: Rust Function
title: test_supported_models
resource: src/llm/provider/openai.rs#L1287-L1292
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/openai/OpenAIProvider/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/openai/OpenAIProvider/provider/supported_models
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn test_supported_models()`

# Calls

- [new](../../../../../functions/src/llm/provider/openai/OpenAIProvider/new.md)
- [supported_models](../../../../../functions/src/llm/provider/openai/OpenAIProvider/provider/supported_models.md)