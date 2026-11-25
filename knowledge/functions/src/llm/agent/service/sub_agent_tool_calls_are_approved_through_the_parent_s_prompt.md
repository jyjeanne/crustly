---
type: Rust Function
title: sub_agent_tool_calls_are_approved_through_the_parent_s_prompt
resource: src/llm/agent/service.rs#L3692-L3736
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
  - target: functions/src/llm/agent/service/agent_registry
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/with_approval_callback
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/send_message_with_tools
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/tool_call
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/text_reply
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/tool_results
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/ScriptedProvider/requests
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn sub_agent_tool_calls_are_approved_through_the_parent_s_prompt()`

# Calls

- [test_context](../../../../../functions/src/llm/agent/service/test_context.md)
- [session_for](../../../../../functions/src/llm/agent/service/session_for.md)
- [new](../../../../../functions/src/llm/agent/service/ScriptedProvider/new.md)
- [new](../../../../../functions/src/llm/agent/service/AgentService/new.md)
- [with_tool_registry](../../../../../functions/src/llm/agent/service/AgentService/with_tool_registry.md)
- [agent_registry](../../../../../functions/src/llm/agent/service/agent_registry.md)
- [with_approval_callback](../../../../../functions/src/llm/agent/service/AgentService/with_approval_callback.md)
- [send_message_with_tools](../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools.md)
- [tool_call](../../../../../functions/src/llm/agent/service/tool_call.md)
- [text_reply](../../../../../functions/src/llm/agent/service/text_reply.md)
- [tool_results](../../../../../functions/src/llm/agent/service/tool_results.md)
- [requests](../../../../../functions/src/llm/agent/service/ScriptedProvider/requests.md)
- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)