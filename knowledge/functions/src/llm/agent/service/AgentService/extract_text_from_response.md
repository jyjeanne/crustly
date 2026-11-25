---
type: Rust Method
title: extract_text_from_response
resource: src/llm/agent/service.rs#L1620-L1637
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/agent/service/AgentService/final_text_and_thinking
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn extract_text_from_response(response: &LLMResponse) -> String`

# Called by

- [final_text_and_thinking](../../../../../../functions/src/llm/agent/service/AgentService/final_text_and_thinking.md)