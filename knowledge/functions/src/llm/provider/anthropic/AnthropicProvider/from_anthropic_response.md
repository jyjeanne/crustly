---
type: Rust Method
title: from_anthropic_response
resource: src/llm/provider/anthropic.rs#L102-L126
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/anthropic/AnthropicProvider/provider/complete
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn from_anthropic_response(&self, response: AnthropicResponse) -> LLMResponse`

# Called by

- [complete](../../../../../../functions/src/llm/provider/anthropic/AnthropicProvider/provider/complete.md)