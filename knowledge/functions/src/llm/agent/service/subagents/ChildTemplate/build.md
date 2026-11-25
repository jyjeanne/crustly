---
type: Rust Method
title: build
resource: src/llm/agent/service/subagents.rs#L62-L100
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/AgentService/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/AgentService/with_tool_registry
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/with_working_directory
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/AgentService/with_allow_sub_agents
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/with_max_tool_iterations
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/with_tool_profile
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/with_auto_approve_tools
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/with_approval_callback
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/subagents/labelled
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/with_verification
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/with_project_notes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/with_fallback
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/with_tool_allowlist
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/with_core_system_prompt
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/service/subagents/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/anthropic/AnthropicProvider/new
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gemini/GeminiProvider/new
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/openai/OpenAIProvider/new
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/openai/OpenAIProvider/local
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/openai/OpenAIProvider/with_base_url
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/QwenProvider/build_client
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/glob/GlobTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/grep/GrepTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/grep/collect_searchable_files
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/ssrf_guard/checked_redirect_policy_blocks_redirect_to_blocked_address
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/web_search/WebSearchTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/logging/init_debug_logging
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/logging/debug_log_files_are_findable_by_the_readers
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/embed/client
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn build(&self, task: &SubAgentTask, profile: &AgentProfile) -> AgentService`

# Calls

- [new](../../../../../../../functions/src/llm/agent/service/AgentService/new.md)
- [with_tool_registry](../../../../../../../functions/src/llm/agent/service/AgentService/with_tool_registry.md)
- [with_working_directory](../../../../../../../functions/src/llm/agent/service/AgentService/with_working_directory.md)
- [with_allow_sub_agents](../../../../../../../functions/src/llm/agent/service/AgentService/with_allow_sub_agents.md)
- [with_max_tool_iterations](../../../../../../../functions/src/llm/agent/service/AgentService/with_max_tool_iterations.md)
- [with_tool_profile](../../../../../../../functions/src/llm/agent/service/AgentService/with_tool_profile.md)
- [with_auto_approve_tools](../../../../../../../functions/src/llm/agent/service/AgentService/with_auto_approve_tools.md)
- [with_approval_callback](../../../../../../../functions/src/llm/agent/service/AgentService/with_approval_callback.md)
- [labelled](../../../../../../../functions/src/llm/agent/service/subagents/labelled.md)
- [with_verification](../../../../../../../functions/src/llm/agent/service/AgentService/with_verification.md)
- [with_project_notes](../../../../../../../functions/src/llm/agent/service/AgentService/with_project_notes.md)
- [with_fallback](../../../../../../../functions/src/llm/agent/service/AgentService/with_fallback.md)
- [with_tool_allowlist](../../../../../../../functions/src/llm/agent/service/AgentService/with_tool_allowlist.md)
- [with_core_system_prompt](../../../../../../../functions/src/llm/agent/service/AgentService/with_core_system_prompt.md)

# Called by

- [execute](../../../../../../../functions/src/llm/agent/service/subagents/execute.md)
- [new](../../../../../../../functions/src/llm/provider/anthropic/AnthropicProvider/new.md)
- [new](../../../../../../../functions/src/llm/provider/gemini/GeminiProvider/new.md)
- [new](../../../../../../../functions/src/llm/provider/openai/OpenAIProvider/new.md)
- [local](../../../../../../../functions/src/llm/provider/openai/OpenAIProvider/local.md)
- [with_base_url](../../../../../../../functions/src/llm/provider/openai/OpenAIProvider/with_base_url.md)
- [build_client](../../../../../../../functions/src/llm/provider/qwen/QwenProvider/build_client.md)
- [execute](../../../../../../../functions/src/llm/tools/glob/GlobTool/tool/execute.md)
- [execute](../../../../../../../functions/src/llm/tools/grep/GrepTool/tool/execute.md)
- [collect_searchable_files](../../../../../../../functions/src/llm/tools/grep/collect_searchable_files.md)
- [checked_redirect_policy_blocks_redirect_to_blocked_address](../../../../../../../functions/src/llm/tools/ssrf_guard/checked_redirect_policy_blocks_redirect_to_blocked_address.md)
- [execute](../../../../../../../functions/src/llm/tools/web_search/WebSearchTool/tool/execute.md)
- [init_debug_logging](../../../../../../../functions/src/logging/init_debug_logging.md)
- [debug_log_files_are_findable_by_the_readers](../../../../../../../functions/src/logging/debug_log_files_are_findable_by_the_readers.md)
- [client](../../../../../../../functions/src/memory/embed/client.md)