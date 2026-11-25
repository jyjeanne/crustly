---
type: Rust Method
title: supported_models
resource: src/llm/provider/openai.rs#L708-L716
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/openai/test_supported_models
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn supported_models(&self) -> Vec<String>`

# Called by

- [test_supported_models](../../../../../../../functions/src/llm/provider/openai/test_supported_models.md)