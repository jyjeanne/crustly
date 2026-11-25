---
type: Rust Function
title: default_auth_style_still_sends_bearer
resource: src/llm/provider/openai.rs#L1274-L1284
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/openai/OpenAIProvider/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/openai/OpenAIProvider/headers
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn default_auth_style_still_sends_bearer()`

# Calls

- [new](../../../../../functions/src/llm/provider/openai/OpenAIProvider/new.md)
- [headers](../../../../../functions/src/llm/provider/openai/OpenAIProvider/headers.md)