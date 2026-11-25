---
type: Rust Method
title: calculate_cost
resource: src/llm/provider/azure.rs#L121-L136
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/provider/azure/test_azure_cost_calculation
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn calculate_cost(&self, model: &str, input_tokens: u32, output_tokens: u32) -> f64`

# Called by

- [test_azure_cost_calculation](../../../../../../../functions/src/llm/provider/azure/test_azure_cost_calculation.md)