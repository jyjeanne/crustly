---
type: Rust Method
title: handle_error
resource: src/llm/provider/anthropic.rs#L129-L186
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/provider/anthropic/AnthropicProvider/provider/complete
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/anthropic/AnthropicProvider/provider/stream
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn handle_error(&self, response: reqwest::Response) -> ProviderError`

# Called by

- [complete](../../../../../../functions/src/llm/provider/anthropic/AnthropicProvider/provider/complete.md)
- [stream](../../../../../../functions/src/llm/provider/anthropic/AnthropicProvider/provider/stream.md)