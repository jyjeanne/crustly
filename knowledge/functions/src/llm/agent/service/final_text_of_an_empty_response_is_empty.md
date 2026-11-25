---
type: Rust Function
title: final_text_of_an_empty_response_is_empty
resource: src/llm/agent/service.rs#L2029-L2035
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/response_with
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/final_text_and_thinking
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn final_text_of_an_empty_response_is_empty()`

# Calls

- [response_with](../../../../../functions/src/llm/agent/service/response_with.md)
- [final_text_and_thinking](../../../../../functions/src/llm/agent/service/AgentService/final_text_and_thinking.md)