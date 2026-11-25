---
type: Rust Method
title: system_prompt_with_env
resource: src/llm/agent/service.rs#L773-L775
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/AgentService/with_env
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/service/AgentService/prepare_message_context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/system_prompt_tells_the_model_the_working_directory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/system_prompt_with_env_is_none_when_no_prompt_is_set
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn system_prompt_with_env(&self) -> Option<String>`

# Calls

- [with_env](../../../../../../functions/src/llm/agent/service/AgentService/with_env.md)

# Called by

- [prepare_message_context](../../../../../../functions/src/llm/agent/service/AgentService/prepare_message_context.md)
- [system_prompt_tells_the_model_the_working_directory](../../../../../../functions/src/llm/agent/service/system_prompt_tells_the_model_the_working_directory.md)
- [system_prompt_with_env_is_none_when_no_prompt_is_set](../../../../../../functions/src/llm/agent/service/system_prompt_with_env_is_none_when_no_prompt_is_set.md)