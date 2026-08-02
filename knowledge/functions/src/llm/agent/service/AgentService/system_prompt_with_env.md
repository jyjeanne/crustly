---
type: Rust Method
title: system_prompt_with_env
resource: src/llm/agent/service.rs#L614-L623
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
  - functions/src/llm/agent/service/AgentService/prepare_message_context
  - functions/src/llm/agent/service/system_prompt_tells_the_model_the_working_directory
---

# Signature

`fn system_prompt_with_env(&self) -> Option<String>`

# Called by

- [send_message_with_tools_inner](../../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)
- [prepare_message_context](../../../../../../functions/src/llm/agent/service/AgentService/prepare_message_context.md)
- [system_prompt_tells_the_model_the_working_directory](../../../../../../functions/src/llm/agent/service/system_prompt_tells_the_model_the_working_directory.md)