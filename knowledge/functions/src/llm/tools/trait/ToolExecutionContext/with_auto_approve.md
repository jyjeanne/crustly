---
type: Rust Method
title: with_auto_approve
resource: src/llm/tools/trait.rs#L164-L167
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/ask_user/test_auto_approve_returns_placeholder
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/bash/test_bash_simple_command
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/bash/test_bash_with_exit_code
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/bash/test_bash_invalid_command
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/bash/test_bash_timeout
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/bash/test_bash_accepts_directory_alias
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/bash/test_bash_timeout_field_overrides_context_default
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/bash/test_bash_is_background_notes_synchronous_fallback
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/http/execute_denies_cloud_metadata_endpoint
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/http/execute_denies_loopback_address
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/powershell/make_ctx
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/registry/test_execute_with_auto_approve
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/trait/test_execution_context
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn with_auto_approve(mut self, auto_approve: bool) -> Self`

# Called by

- [send_message_with_tools_inner](../../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)
- [test_auto_approve_returns_placeholder](../../../../../../functions/src/llm/tools/ask_user/test_auto_approve_returns_placeholder.md)
- [test_bash_simple_command](../../../../../../functions/src/llm/tools/bash/test_bash_simple_command.md)
- [test_bash_with_exit_code](../../../../../../functions/src/llm/tools/bash/test_bash_with_exit_code.md)
- [test_bash_invalid_command](../../../../../../functions/src/llm/tools/bash/test_bash_invalid_command.md)
- [test_bash_timeout](../../../../../../functions/src/llm/tools/bash/test_bash_timeout.md)
- [test_bash_accepts_directory_alias](../../../../../../functions/src/llm/tools/bash/test_bash_accepts_directory_alias.md)
- [test_bash_timeout_field_overrides_context_default](../../../../../../functions/src/llm/tools/bash/test_bash_timeout_field_overrides_context_default.md)
- [test_bash_is_background_notes_synchronous_fallback](../../../../../../functions/src/llm/tools/bash/test_bash_is_background_notes_synchronous_fallback.md)
- [execute_denies_cloud_metadata_endpoint](../../../../../../functions/src/llm/tools/http/execute_denies_cloud_metadata_endpoint.md)
- [execute_denies_loopback_address](../../../../../../functions/src/llm/tools/http/execute_denies_loopback_address.md)
- [make_ctx](../../../../../../functions/src/llm/tools/powershell/make_ctx.md)
- [test_execute_with_auto_approve](../../../../../../functions/src/llm/tools/registry/test_execute_with_auto_approve.md)
- [test_execution_context](../../../../../../functions/src/llm/tools/trait/test_execution_context.md)