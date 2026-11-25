---
type: Rust Function
title: the_router_picks_the_model_for_the_request
resource: src/llm/agent/service.rs#L4059-L4104
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/test_context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/session_for
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/ScriptedProvider/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/AgentService/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/AgentService/with_system_prompt
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/AgentService/with_tool_profile
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/with_model_router
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/router/ModelRouter/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/AgentService/send_message_with_tools
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/ScriptedProvider/requests
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn the_router_picks_the_model_for_the_request()`

# Calls

- [test_context](../../../../../functions/src/llm/agent/service/test_context.md)
- [session_for](../../../../../functions/src/llm/agent/service/session_for.md)
- [new](../../../../../functions/src/llm/agent/service/ScriptedProvider/new.md)
- [new](../../../../../functions/src/llm/agent/service/AgentService/new.md)
- [with_system_prompt](../../../../../functions/src/llm/agent/service/AgentService/with_system_prompt.md)
- [with_tool_profile](../../../../../functions/src/llm/agent/service/AgentService/with_tool_profile.md)
- [with_model_router](../../../../../functions/src/llm/agent/service/AgentService/with_model_router.md)
- [new](../../../../../functions/src/llm/provider/router/ModelRouter/new.md)
- [send_message_with_tools](../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools.md)
- [requests](../../../../../functions/src/llm/agent/service/ScriptedProvider/requests.md)