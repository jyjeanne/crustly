---
type: Rust Function
title: response_with
resource: src/llm/agent/service.rs#L1946-L1959
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/agent/service/final_text_falls_back_to_thinking_when_there_is_no_visible_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/final_text_prefers_visible_text_and_keeps_thinking_separate
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/final_text_of_an_empty_response_is_empty
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn response_with(content: Vec<ContentBlock>) -> LLMResponse`

# Called by

- [final_text_falls_back_to_thinking_when_there_is_no_visible_text](../../../../../functions/src/llm/agent/service/final_text_falls_back_to_thinking_when_there_is_no_visible_text.md)
- [final_text_prefers_visible_text_and_keeps_thinking_separate](../../../../../functions/src/llm/agent/service/final_text_prefers_visible_text_and_keeps_thinking_separate.md)
- [final_text_of_an_empty_response_is_empty](../../../../../functions/src/llm/agent/service/final_text_of_an_empty_response_is_empty.md)