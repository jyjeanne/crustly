---
type: Rust Function
title: test_azure_context_window
resource: src/llm/provider/azure.rs#L157-L167
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
  - target: functions/src/llm/provider/azure/AzureOpenAIProvider/provider/context_window
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn test_azure_context_window()`

# Calls

- [new](../../../../../functions/src/llm/provider/azure/AzureOpenAIProvider/new.md)
- [context_window](../../../../../functions/src/llm/provider/azure/AzureOpenAIProvider/provider/context_window.md)