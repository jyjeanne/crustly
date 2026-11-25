---
type: Rust Function
title: auto_profile_sends_core_tools_on_a_small_window
resource: src/llm/agent/service.rs#L2673-L2727
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
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
  - target: functions/src/llm/agent/service/AgentService/with_tool_registry
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/registry_of
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/with_tool_profile
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/with_system_prompt
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/AgentService/with_core_system_prompt
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/send_message_with_tools
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/ScriptedProvider/requests
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/tool_call
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/text_reply
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/tool_names
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn auto_profile_sends_core_tools_on_a_small_window()`

# Calls

- [test_context](../../../../../functions/src/llm/agent/service/test_context.md)
- [session_for](../../../../../functions/src/llm/agent/service/session_for.md)
- [new](../../../../../functions/src/llm/agent/service/ScriptedProvider/new.md)
- [new](../../../../../functions/src/llm/agent/service/AgentService/new.md)
- [with_tool_registry](../../../../../functions/src/llm/agent/service/AgentService/with_tool_registry.md)
- [registry_of](../../../../../functions/src/llm/agent/service/registry_of.md)
- [with_tool_profile](../../../../../functions/src/llm/agent/service/AgentService/with_tool_profile.md)
- [with_system_prompt](../../../../../functions/src/llm/agent/service/AgentService/with_system_prompt.md)
- [with_core_system_prompt](../../../../../functions/src/llm/agent/service/AgentService/with_core_system_prompt.md)
- [send_message_with_tools](../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools.md)
- [requests](../../../../../functions/src/llm/agent/service/ScriptedProvider/requests.md)
- [tool_call](../../../../../functions/src/llm/agent/service/tool_call.md)
- [text_reply](../../../../../functions/src/llm/agent/service/text_reply.md)
- [tool_names](../../../../../functions/src/llm/agent/service/tool_names.md)
- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)