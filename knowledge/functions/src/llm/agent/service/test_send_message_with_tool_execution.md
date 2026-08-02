---
type: Rust Function
title: test_send_message_with_tool_execution
resource: src/llm/agent/service.rs#L2401-L2436
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/db/Database/run_migrations
  - functions/src/llm/tools/registry/ToolRegistry/register
  - functions/src/llm/agent/service/AgentService/with_tool_registry
  - functions/src/llm/agent/service/AgentService/with_auto_approve_tools
  - functions/src/llm/agent/service/AgentService/send_message_with_tools
---

# Signature

`async fn test_send_message_with_tool_execution()`

# Calls

- [run_migrations](../../../../../functions/src/db/Database/run_migrations.md)
- [register](../../../../../functions/src/llm/tools/registry/ToolRegistry/register.md)
- [with_tool_registry](../../../../../functions/src/llm/agent/service/AgentService/with_tool_registry.md)
- [with_auto_approve_tools](../../../../../functions/src/llm/agent/service/AgentService/with_auto_approve_tools.md)
- [send_message_with_tools](../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools.md)