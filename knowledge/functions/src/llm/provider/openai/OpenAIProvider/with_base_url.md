---
type: Rust Method
title: with_base_url
resource: src/llm/provider/openai.rs#L108-L124
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/subagents/ChildTemplate/build
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/azure/AzureOpenAIProvider/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/azure/AzureOpenAIProvider/with_endpoint
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/openai/with_api_key_header_sends_api_key_not_bearer
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn with_base_url(api_key: String, base_url: String) -> Self`

# Calls

- [build](../../../../../../functions/src/llm/agent/service/subagents/ChildTemplate/build.md)

# Called by

- [new](../../../../../../functions/src/llm/provider/azure/AzureOpenAIProvider/new.md)
- [with_endpoint](../../../../../../functions/src/llm/provider/azure/AzureOpenAIProvider/with_endpoint.md)
- [with_api_key_header_sends_api_key_not_bearer](../../../../../../functions/src/llm/provider/openai/with_api_key_header_sends_api_key_not_bearer.md)