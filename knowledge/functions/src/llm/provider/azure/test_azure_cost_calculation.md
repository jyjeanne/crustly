---
type: Rust Function
title: test_azure_cost_calculation
resource: src/llm/provider/azure.rs#L170-L182
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
  - target: functions/src/llm/provider/azure/AzureOpenAIProvider/provider/calculate_cost
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn test_azure_cost_calculation()`

# Calls

- [new](../../../../../functions/src/llm/provider/azure/AzureOpenAIProvider/new.md)
- [calculate_cost](../../../../../functions/src/llm/provider/azure/AzureOpenAIProvider/provider/calculate_cost.md)