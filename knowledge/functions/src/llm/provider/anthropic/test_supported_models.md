---
type: Rust Function
title: test_supported_models
resource: src/llm/provider/anthropic.rs#L507-L512
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/anthropic/AnthropicProvider/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/anthropic/AnthropicProvider/provider/supported_models
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn test_supported_models()`

# Calls

- [new](../../../../../functions/src/llm/provider/anthropic/AnthropicProvider/new.md)
- [supported_models](../../../../../functions/src/llm/provider/anthropic/AnthropicProvider/provider/supported_models.md)