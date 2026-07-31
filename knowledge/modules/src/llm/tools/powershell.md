---
type: Rust Module
title: powershell
resource: src/llm/tools/powershell.rs#L1-L524
generated:
  by: okf-rs/0.2.0
relationships:
  imports:
  - external/super-error-result-toolerror
  - external/super-r-trait-tool-toolcapability-toolexecutioncontext-toolresult
  - external/async-trait-async-trait
  - external/once-cell-sync-lazy
  - external/serde-deserialize-serialize
  - external/serde-json-value
  - external/std-collections-hashmap
  - external/tokio-process-command
  - external/tokio-time-timeout-duration
  - external/super
  - external/uuid-uuid
  member_of:
  - packages/crustly
---

# Contains

- [probe_executable](../../../../functions/src/llm/tools/powershell/probe_executable.md)
- [is_read_only_powershell](../../../../functions/src/llm/tools/powershell/is_read_only_powershell.md)
- [PowerShellTool](../../../../classes/src/llm/tools/powershell/PowerShellTool.md)
- [PowerShellInput](../../../../classes/src/llm/tools/powershell/PowerShellInput.md)
- [name](../../../../functions/src/llm/tools/powershell/PowerShellTool/tool/name.md)
- [description](../../../../functions/src/llm/tools/powershell/PowerShellTool/tool/description.md)
- [input_schema](../../../../functions/src/llm/tools/powershell/PowerShellTool/tool/input_schema.md)
- [capabilities](../../../../functions/src/llm/tools/powershell/PowerShellTool/tool/capabilities.md)
- [requires_approval](../../../../functions/src/llm/tools/powershell/PowerShellTool/tool/requires_approval.md)
- [validate_input](../../../../functions/src/llm/tools/powershell/PowerShellTool/tool/validate_input.md)
- [execute](../../../../functions/src/llm/tools/powershell/PowerShellTool/tool/execute.md)
- [make_ctx](../../../../functions/src/llm/tools/powershell/make_ctx.md)
- [read_only_allows_get_content](../../../../functions/src/llm/tools/powershell/read_only_allows_get_content.md)
- [read_only_allows_get_childitem](../../../../functions/src/llm/tools/powershell/read_only_allows_get_childitem.md)
- [read_only_allows_select_string](../../../../functions/src/llm/tools/powershell/read_only_allows_select_string.md)
- [read_only_blocks_remove_item](../../../../functions/src/llm/tools/powershell/read_only_blocks_remove_item.md)
- [read_only_blocks_invoke_expression](../../../../functions/src/llm/tools/powershell/read_only_blocks_invoke_expression.md)
- [read_only_blocks_pipe_to_out_file](../../../../functions/src/llm/tools/powershell/read_only_blocks_pipe_to_out_file.md)
- [read_only_blocks_net_method_call](../../../../functions/src/llm/tools/powershell/read_only_blocks_net_method_call.md)
- [read_only_blocks_iex_without_space](../../../../functions/src/llm/tools/powershell/read_only_blocks_iex_without_space.md)
- [read_only_allows_gt_in_string_argument](../../../../functions/src/llm/tools/powershell/read_only_allows_gt_in_string_argument.md)
- [read_only_blocks_redirection_with_space](../../../../functions/src/llm/tools/powershell/read_only_blocks_redirection_with_space.md)
- [read_only_blocks_append_no_spaces](../../../../functions/src/llm/tools/powershell/read_only_blocks_append_no_spaces.md)
- [validate_rejects_empty_command](../../../../functions/src/llm/tools/powershell/validate_rejects_empty_command.md)
- [validate_rejects_zero_timeout](../../../../functions/src/llm/tools/powershell/validate_rejects_zero_timeout.md)
- [validate_rejects_timeout_over_600](../../../../functions/src/llm/tools/powershell/validate_rejects_timeout_over_600.md)
- [validate_accepts_valid_input](../../../../functions/src/llm/tools/powershell/validate_accepts_valid_input.md)
- [execute_blocks_dangerous_command_in_read_only_mode](../../../../functions/src/llm/tools/powershell/execute_blocks_dangerous_command_in_read_only_mode.md)
- [execute_allows_read_only_command_in_plan_mode](../../../../functions/src/llm/tools/powershell/execute_allows_read_only_command_in_plan_mode.md)
- [tool_metadata](../../../../functions/src/llm/tools/powershell/tool_metadata.md)

# Imports

- `super::error::{Result, ToolError}`
- `super::r#trait::{Tool, ToolCapability, ToolExecutionContext, ToolResult}`
- `async_trait::async_trait`
- `once_cell::sync::Lazy`
- `serde::{Deserialize, Serialize}`
- `serde_json::Value`
- `std::collections::HashMap`
- `tokio::process::Command`
- `tokio::time::{timeout, Duration}`
- `super::*`
- `uuid::Uuid`

# Member of

- [crustly](../../../../packages/crustly.md)