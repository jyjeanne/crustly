---
type: Rust Function
title: test_azure_provider_creation
resource: src/llm/provider/azure.rs#L144-L154
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/azure/AzureOpenAIProvider/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/azure/AzureOpenAIProvider/provider/name
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/trait/Provider/supports_streaming
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/trait/Provider/supports_tools
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn test_azure_provider_creation()`

# Calls

- [new](../../../../../functions/src/llm/provider/azure/AzureOpenAIProvider/new.md)
- [name](../../../../../functions/src/llm/provider/azure/AzureOpenAIProvider/provider/name.md)
- [supports_streaming](../../../../../functions/src/llm/provider/trait/Provider/supports_streaming.md)
- [supports_tools](../../../../../functions/src/llm/provider/trait/Provider/supports_tools.md)