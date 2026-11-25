---
type: Rust Method
title: final_text_and_thinking
resource: src/llm/agent/service.rs#L1671-L1690
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/AgentService/extract_text_from_response
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/extract_thinking_from_response
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/service/AgentService/send_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
    resolved_by: tree-sitter
    confidence: exact
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

`fn final_text_and_thinking(response: &LLMResponse) -> FinalText`

# Calls

- [extract_text_from_response](../../../../../../functions/src/llm/agent/service/AgentService/extract_text_from_response.md)
- [extract_thinking_from_response](../../../../../../functions/src/llm/agent/service/AgentService/extract_thinking_from_response.md)

# Called by

- [send_message](../../../../../../functions/src/llm/agent/service/AgentService/send_message.md)
- [send_message_with_tools_inner](../../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)
- [final_text_falls_back_to_thinking_when_there_is_no_visible_text](../../../../../../functions/src/llm/agent/service/final_text_falls_back_to_thinking_when_there_is_no_visible_text.md)
- [final_text_prefers_visible_text_and_keeps_thinking_separate](../../../../../../functions/src/llm/agent/service/final_text_prefers_visible_text_and_keeps_thinking_separate.md)
- [final_text_of_an_empty_response_is_empty](../../../../../../functions/src/llm/agent/service/final_text_of_an_empty_response_is_empty.md)