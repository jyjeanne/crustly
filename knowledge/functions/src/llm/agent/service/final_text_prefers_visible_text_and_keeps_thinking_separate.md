---
type: Rust Function
title: final_text_prefers_visible_text_and_keeps_thinking_separate
resource: src/llm/agent/service.rs#L1994-L2010
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
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

`fn final_text_prefers_visible_text_and_keeps_thinking_separate()`

# Calls

- [response_with](../../../../../functions/src/llm/agent/service/response_with.md)
- [final_text_and_thinking](../../../../../functions/src/llm/agent/service/AgentService/final_text_and_thinking.md)