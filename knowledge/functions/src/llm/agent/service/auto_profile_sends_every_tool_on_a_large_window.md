---
type: Rust Function
title: auto_profile_sends_every_tool_on_a_large_window
resource: src/llm/agent/service.rs#L2744-L2762
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
  - target: functions/src/llm/agent/service/AgentService/with_tool_registry
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/registry_of
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/with_tool_profile
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/send_message_with_tools
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/text_reply
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/tool_names
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/ScriptedProvider/requests
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/exposed_tool_names
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn auto_profile_sends_every_tool_on_a_large_window()`

# Calls

- [test_context](../../../../../functions/src/llm/agent/service/test_context.md)
- [session_for](../../../../../functions/src/llm/agent/service/session_for.md)
- [new](../../../../../functions/src/llm/agent/service/ScriptedProvider/new.md)
- [new](../../../../../functions/src/llm/agent/service/AgentService/new.md)
- [with_tool_registry](../../../../../functions/src/llm/agent/service/AgentService/with_tool_registry.md)
- [registry_of](../../../../../functions/src/llm/agent/service/registry_of.md)
- [with_tool_profile](../../../../../functions/src/llm/agent/service/AgentService/with_tool_profile.md)
- [send_message_with_tools](../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools.md)
- [text_reply](../../../../../functions/src/llm/agent/service/text_reply.md)
- [tool_names](../../../../../functions/src/llm/agent/service/tool_names.md)
- [requests](../../../../../functions/src/llm/agent/service/ScriptedProvider/requests.md)
- [exposed_tool_names](../../../../../functions/src/llm/agent/service/AgentService/exposed_tool_names.md)