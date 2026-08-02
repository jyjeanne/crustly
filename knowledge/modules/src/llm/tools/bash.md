---
type: Rust Module
title: bash
resource: src/llm/tools/bash.rs#L1-L735
generated:
  by: okf-rs/0.3.0
relationships:
  imports:
  - external/super-error-result-toolerror
  - external/super-r-trait-tool-toolcapability-toolexecutioncontext-toolresult
  - external/async-trait-async-trait
  - external/serde-deserialize-serialize
  - external/serde-json-value
  - external/tokio-process-command
  - external/tokio-time-timeout-duration
  - external/std-sync-oncelock
  - external/super
  - external/uuid-uuid
  member_of:
  - packages/crustly
---

# Contains

- [resolve_shell](../../../../functions/src/llm/tools/bash/resolve_shell.md)
- [BashTool](../../../../classes/src/llm/tools/bash/BashTool.md)
- [BashInput](../../../../classes/src/llm/tools/bash/BashInput.md)
- [is_read_only_command](../../../../functions/src/llm/tools/bash/is_read_only_command.md)
- [name](../../../../functions/src/llm/tools/bash/BashTool/tool/name.md)
- [description](../../../../functions/src/llm/tools/bash/BashTool/tool/description.md)
- [input_schema](../../../../functions/src/llm/tools/bash/BashTool/tool/input_schema.md)
- [capabilities](../../../../functions/src/llm/tools/bash/BashTool/tool/capabilities.md)
- [requires_approval](../../../../functions/src/llm/tools/bash/BashTool/tool/requires_approval.md)
- [validate_input](../../../../functions/src/llm/tools/bash/BashTool/tool/validate_input.md)
- [execute](../../../../functions/src/llm/tools/bash/BashTool/tool/execute.md)
- [windows_resolves_a_posix_shell_not_cmd](../../../../functions/src/llm/tools/bash/windows_resolves_a_posix_shell_not_cmd.md)
- [bash_runs_posix_in_the_requested_working_directory](../../../../functions/src/llm/tools/bash/bash_runs_posix_in_the_requested_working_directory.md)
- [test_bash_simple_command](../../../../functions/src/llm/tools/bash/test_bash_simple_command.md)
- [test_bash_with_exit_code](../../../../functions/src/llm/tools/bash/test_bash_with_exit_code.md)
- [test_bash_invalid_command](../../../../functions/src/llm/tools/bash/test_bash_invalid_command.md)
- [test_bash_timeout](../../../../functions/src/llm/tools/bash/test_bash_timeout.md)
- [test_bash_tool_schema](../../../../functions/src/llm/tools/bash/test_bash_tool_schema.md)
- [test_bash_accepts_directory_alias](../../../../functions/src/llm/tools/bash/test_bash_accepts_directory_alias.md)
- [test_bash_timeout_field_overrides_context_default](../../../../functions/src/llm/tools/bash/test_bash_timeout_field_overrides_context_default.md)
- [test_bash_is_background_notes_synchronous_fallback](../../../../functions/src/llm/tools/bash/test_bash_is_background_notes_synchronous_fallback.md)
- [test_validate_empty_command](../../../../functions/src/llm/tools/bash/test_validate_empty_command.md)
- [read_only_mode_rejects_chained_destructive_commands](../../../../functions/src/llm/tools/bash/read_only_mode_rejects_chained_destructive_commands.md)
- [read_only_mode_rejects_network_fetch_tools](../../../../functions/src/llm/tools/bash/read_only_mode_rejects_network_fetch_tools.md)
- [read_only_mode_allows_simple_safe_commands](../../../../functions/src/llm/tools/bash/read_only_mode_allows_simple_safe_commands.md)
- [read_only_mode_rejects_mutating_find_flags](../../../../functions/src/llm/tools/bash/read_only_mode_rejects_mutating_find_flags.md)
- [read_only_mode_rejects_git_config](../../../../functions/src/llm/tools/bash/read_only_mode_rejects_git_config.md)

# Imports

- `super::error::{Result, ToolError}`
- `super::r#trait::{Tool, ToolCapability, ToolExecutionContext, ToolResult}`
- `async_trait::async_trait`
- `serde::{Deserialize, Serialize}`
- `serde_json::Value`
- `tokio::process::Command`
- `tokio::time::{timeout, Duration}`
- `std::sync::OnceLock`
- `super::*`
- `uuid::Uuid`

# Member of

- [crustly](../../../../packages/crustly.md)