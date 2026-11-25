---
type: Rust Function
title: a_refused_global_save_stores_nothing
resource: src/llm/agent/service.rs#L4279-L4314
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
  - target: functions/src/llm/agent/service/memory_test_store
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/registry/ToolRegistry/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/registry/ToolRegistry/register
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/save_memory/SaveMemoryTool/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/ScriptedProvider/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/AgentService/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/AgentService/with_tool_registry
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/with_approval_callback
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/send_message_with_tools
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/ServiceContext/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/tool_call
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/text_reply
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/take_notices
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn a_refused_global_save_stores_nothing()`

# Calls

- [test_context](../../../../../functions/src/llm/agent/service/test_context.md)
- [session_for](../../../../../functions/src/llm/agent/service/session_for.md)
- [memory_test_store](../../../../../functions/src/llm/agent/service/memory_test_store.md)
- [new](../../../../../functions/src/llm/tools/registry/ToolRegistry/new.md)
- [register](../../../../../functions/src/llm/tools/registry/ToolRegistry/register.md)
- [new](../../../../../functions/src/llm/tools/save_memory/SaveMemoryTool/new.md)
- [new](../../../../../functions/src/llm/agent/service/ScriptedProvider/new.md)
- [new](../../../../../functions/src/llm/agent/service/AgentService/new.md)
- [with_tool_registry](../../../../../functions/src/llm/agent/service/AgentService/with_tool_registry.md)
- [with_approval_callback](../../../../../functions/src/llm/agent/service/AgentService/with_approval_callback.md)
- [send_message_with_tools](../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools.md)
- [pool](../../../../../functions/src/services/ServiceContext/pool.md)
- [tool_call](../../../../../functions/src/llm/agent/service/tool_call.md)
- [text_reply](../../../../../functions/src/llm/agent/service/text_reply.md)
- [take_notices](../../../../../functions/src/memory/MemoryStore/take_notices.md)