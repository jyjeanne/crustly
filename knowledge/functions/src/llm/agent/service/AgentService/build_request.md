---
type: Rust Method
title: build_request
resource: src/llm/agent/service.rs#L1067-L1082
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/types/LLMRequest/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/types/LLMRequest/with_max_tokens
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/context/AgentContext/effective_system_prompt
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/LLMRequest/with_system
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/LLMRequest/with_tools
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/service/AgentService/prepare_message_context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/run/AgentRun/call_model
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/run/AgentRun/finish
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn build_request( model_name: &str, context: &AgentContext, tools: &RequestTools, max_tokens: u32, ) -> LLMRequest`

# Calls

- [new](../../../../../../functions/src/llm/provider/types/LLMRequest/new.md)
- [with_max_tokens](../../../../../../functions/src/llm/provider/types/LLMRequest/with_max_tokens.md)
- [effective_system_prompt](../../../../../../functions/src/llm/agent/context/AgentContext/effective_system_prompt.md)
- [with_system](../../../../../../functions/src/llm/provider/types/LLMRequest/with_system.md)
- [with_tools](../../../../../../functions/src/llm/provider/types/LLMRequest/with_tools.md)

# Called by

- [prepare_message_context](../../../../../../functions/src/llm/agent/service/AgentService/prepare_message_context.md)
- [call_model](../../../../../../functions/src/llm/agent/service/run/AgentRun/call_model.md)
- [finish](../../../../../../functions/src/llm/agent/service/run/AgentRun/finish.md)