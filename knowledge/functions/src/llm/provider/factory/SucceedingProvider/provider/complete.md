---
type: Rust Method
title: complete
resource: src/llm/provider/factory.rs#L622-L638
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/provider/factory/ModelRecorder/provider/complete
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn complete(&self, _req: LLMRequest) -> ProviderResult<LLMResponse>`

# Called by

- [complete](../../../../../../../functions/src/llm/provider/factory/ModelRecorder/provider/complete.md)