---
type: Rust Function
title: with_api_key_header_sends_api_key_not_bearer
resource: src/llm/provider/openai.rs#L1253-L1270
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/openai/OpenAIProvider/with_base_url
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/openai/OpenAIProvider/with_api_key_header
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/openai/OpenAIProvider/headers
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn with_api_key_header_sends_api_key_not_bearer()`

# Calls

- [with_base_url](../../../../../functions/src/llm/provider/openai/OpenAIProvider/with_base_url.md)
- [with_api_key_header](../../../../../functions/src/llm/provider/openai/OpenAIProvider/with_api_key_header.md)
- [headers](../../../../../functions/src/llm/provider/openai/OpenAIProvider/headers.md)