---
type: Rust Method
title: user
resource: src/llm/provider/types.rs#L31-L36
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/llm/agent/context/test_add_message
  - functions/src/llm/agent/context/test_would_exceed_limit
  - functions/src/llm/agent/context/test_usage_percentage
  - functions/src/llm/agent/context/test_trim_to_fit
  - functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
  - functions/src/llm/agent/service/AgentService/prepare_message_context
  - functions/src/llm/provider/types/test_message_creation
---

# Signature

`pub fn user(text: impl Into<String>) -> Self`

# Called by

- [test_add_message](../../../../../../functions/src/llm/agent/context/test_add_message.md)
- [test_would_exceed_limit](../../../../../../functions/src/llm/agent/context/test_would_exceed_limit.md)
- [test_usage_percentage](../../../../../../functions/src/llm/agent/context/test_usage_percentage.md)
- [test_trim_to_fit](../../../../../../functions/src/llm/agent/context/test_trim_to_fit.md)
- [send_message_with_tools_inner](../../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)
- [prepare_message_context](../../../../../../functions/src/llm/agent/service/AgentService/prepare_message_context.md)
- [test_message_creation](../../../../../../functions/src/llm/provider/types/test_message_creation.md)