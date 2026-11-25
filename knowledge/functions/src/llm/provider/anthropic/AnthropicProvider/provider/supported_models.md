---
type: Rust Method
title: supported_models
resource: src/llm/provider/anthropic.rs#L309-L316
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/provider/anthropic/test_supported_models
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn supported_models(&self) -> Vec<String>`

# Called by

- [test_supported_models](../../../../../../../functions/src/llm/provider/anthropic/test_supported_models.md)