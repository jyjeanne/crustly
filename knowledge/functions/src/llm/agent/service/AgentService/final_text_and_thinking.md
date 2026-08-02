---
type: Rust Method
title: final_text_and_thinking
resource: src/llm/agent/service.rs#L1698-L1717
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/agent/service/AgentService/extract_text_from_response
  - functions/src/llm/agent/service/AgentService/extract_thinking_from_response
  - functions/src/config/secrets/SecretString/is_empty
  called_by:
  - functions/src/llm/agent/service/AgentService/send_message
  - functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
  - functions/src/llm/agent/service/final_text_falls_back_to_thinking_when_there_is_no_visible_text
  - functions/src/llm/agent/service/final_text_prefers_visible_text_and_keeps_thinking_separate
  - functions/src/llm/agent/service/final_text_of_an_empty_response_is_empty
---

# Signature

`fn final_text_and_thinking(response: &LLMResponse) -> FinalText`

# Calls

- [extract_text_from_response](../../../../../../functions/src/llm/agent/service/AgentService/extract_text_from_response.md)
- [extract_thinking_from_response](../../../../../../functions/src/llm/agent/service/AgentService/extract_thinking_from_response.md)
- [is_empty](../../../../../../functions/src/config/secrets/SecretString/is_empty.md)

# Called by

- [send_message](../../../../../../functions/src/llm/agent/service/AgentService/send_message.md)
- [send_message_with_tools_inner](../../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)
- [final_text_falls_back_to_thinking_when_there_is_no_visible_text](../../../../../../functions/src/llm/agent/service/final_text_falls_back_to_thinking_when_there_is_no_visible_text.md)
- [final_text_prefers_visible_text_and_keeps_thinking_separate](../../../../../../functions/src/llm/agent/service/final_text_prefers_visible_text_and_keeps_thinking_separate.md)
- [final_text_of_an_empty_response_is_empty](../../../../../../functions/src/llm/agent/service/final_text_of_an_empty_response_is_empty.md)