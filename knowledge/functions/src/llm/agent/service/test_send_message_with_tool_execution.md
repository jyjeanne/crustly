---
type: Rust Function
title: test_send_message_with_tool_execution
resource: src/llm/agent/service.rs#L2252-L2287
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/db/Database/connect_in_memory
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/Database/run_migrations
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/Database/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/ServiceContext/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/MockProviderWithTools/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/registry/ToolRegistry/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/registry/ToolRegistry/register
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/AgentService/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/AgentService/with_tool_registry
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/with_auto_approve_tools
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/session/SessionService/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/session/SessionService/create_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/send_message_with_tools
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn test_send_message_with_tool_execution()`

# Calls

- [connect_in_memory](../../../../../functions/src/db/Database/connect_in_memory.md)
- [run_migrations](../../../../../functions/src/db/Database/run_migrations.md)
- [pool](../../../../../functions/src/db/Database/pool.md)
- [new](../../../../../functions/src/services/ServiceContext/new.md)
- [new](../../../../../functions/src/llm/agent/service/MockProviderWithTools/new.md)
- [new](../../../../../functions/src/llm/tools/registry/ToolRegistry/new.md)
- [register](../../../../../functions/src/llm/tools/registry/ToolRegistry/register.md)
- [new](../../../../../functions/src/llm/agent/service/AgentService/new.md)
- [with_tool_registry](../../../../../functions/src/llm/agent/service/AgentService/with_tool_registry.md)
- [with_auto_approve_tools](../../../../../functions/src/llm/agent/service/AgentService/with_auto_approve_tools.md)
- [new](../../../../../functions/src/services/session/SessionService/new.md)
- [create_session](../../../../../functions/src/services/session/SessionService/create_session.md)
- [send_message_with_tools](../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools.md)