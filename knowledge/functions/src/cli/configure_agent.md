---
type: Rust Function
title: configure_agent
resource: src/cli/mod.rs#L159-L195
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/cli/configure_agent_loop
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/with_subagents
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/profiles/resolve_profiles
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/provider_name
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/AgentService/provider_model
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/AgentService/with_model_router
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/router/ModelRouter/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/llama_cpp_download/model
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/factory/create_named_provider
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/error/ErrorInfo/with_context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/with_fallback
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/cmd_chat
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/cmd_run
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn configure_agent( service: crate::llm::agent::AgentService, config: &crate::config::Config, can_approve_shell: bool, ) -> Result<crate::llm::agent::AgentService>`

# Calls

- [configure_agent_loop](../../../functions/src/cli/configure_agent_loop.md)
- [with_subagents](../../../functions/src/llm/agent/service/AgentService/with_subagents.md)
- [resolve_profiles](../../../functions/src/llm/agent/profiles/resolve_profiles.md)
- [provider_name](../../../functions/src/llm/agent/service/AgentService/provider_name.md)
- [provider_model](../../../functions/src/llm/agent/service/AgentService/provider_model.md)
- [with_model_router](../../../functions/src/llm/agent/service/AgentService/with_model_router.md)
- [new](../../../functions/src/llm/provider/router/ModelRouter/new.md)
- [model](../../../functions/src/tui/llama_cpp_download/model.md)
- [create_named_provider](../../../functions/src/llm/provider/factory/create_named_provider.md)
- [with_context](../../../functions/src/tui/error/ErrorInfo/with_context.md)
- [with_fallback](../../../functions/src/llm/agent/service/AgentService/with_fallback.md)

# Called by

- [cmd_chat](../../../functions/src/cli/cmd_chat.md)
- [cmd_run](../../../functions/src/cli/cmd_run.md)