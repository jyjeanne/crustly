---
type: Rust Function
title: final_text_falls_back_to_thinking_when_there_is_no_visible_text
resource: src/llm/agent/service.rs#L1973-L2003
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

`fn final_text_falls_back_to_thinking_when_there_is_no_visible_text()`

# Calls

- [response_with](../../../../../functions/src/llm/agent/service/response_with.md)
- [final_text_and_thinking](../../../../../functions/src/llm/agent/service/AgentService/final_text_and_thinking.md)