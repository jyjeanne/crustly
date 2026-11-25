---
type: Rust Function
title: test_azure_supported_models
resource: src/llm/provider/azure.rs#L185-L195
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/azure/AzureOpenAIProvider/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/azure/AzureOpenAIProvider/provider/supported_models
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn test_azure_supported_models()`

# Calls

- [new](../../../../../functions/src/llm/provider/azure/AzureOpenAIProvider/new.md)
- [supported_models](../../../../../functions/src/llm/provider/azure/AzureOpenAIProvider/provider/supported_models.md)