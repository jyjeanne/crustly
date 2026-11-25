---
type: Rust Function
title: cmd_run
resource: src/cli/mod.rs#L2194-L2357
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/Database/connect
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/Database/run_migrations
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/factory/create_provider
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/memory_store
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/Database/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/cli/build_tool_registry
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/registry/ToolRegistry/register_mcp_server
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/registry/ToolRegistry/set_policy
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/SecurityConfig/to_policy
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/ServiceContext/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/cli/configure_agent
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/AgentService/with_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/with_tool_registry
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/with_system_prompt
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/AgentService/with_auto_approve_tools
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/context_usage_preview
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/ContextUsage/overhead_is_tight
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
  - target: functions/src/memory/project/identify
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/extraction/MemoryStore/extract
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/take_notices
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/describe_context_usage
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/cmd_autoplan
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn cmd_run( config: &crate::config::Config, prompt: String, auto_approve: bool, format: OutputFormat, ) -> Result<()>`

# Calls

- [connect](../../../functions/src/db/Database/connect.md)
- [run_migrations](../../../functions/src/db/Database/run_migrations.md)
- [create_provider](../../../functions/src/llm/provider/factory/create_provider.md)
- [memory_store](../../../functions/src/cli/memory_store.md)
- [pool](../../../functions/src/db/Database/pool.md)
- [build_tool_registry](../../../functions/src/cli/build_tool_registry.md)
- [register_mcp_server](../../../functions/src/llm/tools/registry/ToolRegistry/register_mcp_server.md)
- [set_policy](../../../functions/src/llm/tools/registry/ToolRegistry/set_policy.md)
- [to_policy](../../../functions/src/config/SecurityConfig/to_policy.md)
- [new](../../../functions/src/services/ServiceContext/new.md)
- [configure_agent](../../../functions/src/cli/configure_agent.md)
- [new](../../../functions/src/llm/agent/service/AgentService/new.md)
- [with_memory](../../../functions/src/llm/agent/service/AgentService/with_memory.md)
- [with_tool_registry](../../../functions/src/llm/agent/service/AgentService/with_tool_registry.md)
- [with_system_prompt](../../../functions/src/llm/agent/service/AgentService/with_system_prompt.md)
- [with_auto_approve_tools](../../../functions/src/llm/agent/service/AgentService/with_auto_approve_tools.md)
- [context_usage_preview](../../../functions/src/llm/agent/service/AgentService/context_usage_preview.md)
- [overhead_is_tight](../../../functions/src/llm/agent/budget/ContextUsage/overhead_is_tight.md)
- [new](../../../functions/src/services/session/SessionService/new.md)
- [create_session](../../../functions/src/services/session/SessionService/create_session.md)
- [send_message_with_tools](../../../functions/src/llm/agent/service/AgentService/send_message_with_tools.md)
- [identify](../../../functions/src/memory/project/identify.md)
- [extract](../../../functions/src/memory/extraction/MemoryStore/extract.md)
- [take_notices](../../../functions/src/memory/MemoryStore/take_notices.md)
- [describe_context_usage](../../../functions/src/cli/describe_context_usage.md)

# Called by

- [run](../../../functions/src/cli/run.md)
- [cmd_autoplan](../../../functions/src/cli/cmd_autoplan.md)