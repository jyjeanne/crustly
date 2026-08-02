---
type: Rust Method
title: extract_text_from_response
resource: src/llm/agent/service.rs#L1661-L1678
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/llm/agent/service/AgentService/final_text_and_thinking
---

# Signature

`fn extract_text_from_response(response: &LLMResponse) -> String`

# Called by

- [final_text_and_thinking](../../../../../../functions/src/llm/agent/service/AgentService/final_text_and_thinking.md)