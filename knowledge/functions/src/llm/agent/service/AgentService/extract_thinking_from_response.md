---
type: Rust Method
title: extract_thinking_from_response
resource: src/llm/agent/service.rs#L1693-L1708
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/agent/service/AgentService/final_text_and_thinking
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn extract_thinking_from_response(response: &LLMResponse) -> Option<String>`

# Called by

- [final_text_and_thinking](../../../../../../functions/src/llm/agent/service/AgentService/final_text_and_thinking.md)