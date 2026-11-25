---
type: Rust Method
title: to_anthropic_request
resource: src/llm/provider/anthropic.rs#L86-L98
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/anthropic/AnthropicProvider/provider/complete
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/anthropic/AnthropicProvider/provider/stream
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn to_anthropic_request(&self, request: LLMRequest) -> AnthropicRequest`

# Called by

- [complete](../../../../../../functions/src/llm/provider/anthropic/AnthropicProvider/provider/complete.md)
- [stream](../../../../../../functions/src/llm/provider/anthropic/AnthropicProvider/provider/stream.md)