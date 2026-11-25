---
type: Rust Function
title: build_tool_registry
resource: src/cli/mod.rs#L1825-L1876
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/tools/registry/ToolRegistry/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/registry/ToolRegistry/register
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/search_memory/SearchMemoryTool/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/forget_memory/ForgetMemoryTool/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/save_memory/SaveMemoryTool/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/cli/cmd_chat
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/cmd_run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/autoplan_prompts_for_the_registered_code_and_shell_tool_names
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/core_profile_fixed_overhead_fits_a_small_window
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/build_tool_registry_registers_every_built_in_tool
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/connect_configured_mcp_servers_returns_empty_status_with_no_servers
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/connect_configured_mcp_servers_records_failure_for_unreachable_server
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn build_tool_registry( memory: Option<Arc<crate::memory::MemoryStore>>, ) -> crate::llm::tools::registry::ToolRegistry`

# Calls

- [new](../../../functions/src/llm/tools/registry/ToolRegistry/new.md)
- [register](../../../functions/src/llm/tools/registry/ToolRegistry/register.md)
- [new](../../../functions/src/llm/tools/search_memory/SearchMemoryTool/new.md)
- [new](../../../functions/src/llm/tools/forget_memory/ForgetMemoryTool/new.md)
- [new](../../../functions/src/llm/tools/save_memory/SaveMemoryTool/new.md)

# Called by

- [cmd_chat](../../../functions/src/cli/cmd_chat.md)
- [cmd_run](../../../functions/src/cli/cmd_run.md)
- [autoplan_prompts_for_the_registered_code_and_shell_tool_names](../../../functions/src/cli/autoplan_prompts_for_the_registered_code_and_shell_tool_names.md)
- [core_profile_fixed_overhead_fits_a_small_window](../../../functions/src/cli/core_profile_fixed_overhead_fits_a_small_window.md)
- [build_tool_registry_registers_every_built_in_tool](../../../functions/src/cli/build_tool_registry_registers_every_built_in_tool.md)
- [connect_configured_mcp_servers_returns_empty_status_with_no_servers](../../../functions/src/cli/connect_configured_mcp_servers_returns_empty_status_with_no_servers.md)
- [connect_configured_mcp_servers_records_failure_for_unreachable_server](../../../functions/src/cli/connect_configured_mcp_servers_records_failure_for_unreachable_server.md)