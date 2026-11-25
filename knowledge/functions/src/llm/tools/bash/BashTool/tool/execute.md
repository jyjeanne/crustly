---
type: Rust Method
title: execute
resource: src/llm/tools/bash.rs#L309-L426
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/tools/bash/is_read_only_command
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/trait/ToolResult/error
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/code_knowledge/CodeKnowledge/exists
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/bash/resolve_shell
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/error/CrustlyError/code
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/trait/ToolResult/success
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/trait/ToolResult/with_metadata
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/SkillSource/display
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/bash/bash_runs_posix_in_the_requested_working_directory
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/bash/test_bash_simple_command
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/bash/test_bash_with_exit_code
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/bash/test_bash_invalid_command
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/bash/test_bash_timeout
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/bash/test_bash_accepts_directory_alias
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/bash/test_bash_timeout_field_overrides_context_default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/bash/test_bash_is_background_notes_synchronous_fallback
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>`

# Calls

- [is_read_only_command](../../../../../../../functions/src/llm/tools/bash/is_read_only_command.md)
- [error](../../../../../../../functions/src/llm/tools/trait/ToolResult/error.md)
- [exists](../../../../../../../functions/src/memory/code_knowledge/CodeKnowledge/exists.md)
- [resolve_shell](../../../../../../../functions/src/llm/tools/bash/resolve_shell.md)
- [code](../../../../../../../functions/src/error/CrustlyError/code.md)
- [success](../../../../../../../functions/src/llm/tools/trait/ToolResult/success.md)
- [with_metadata](../../../../../../../functions/src/llm/tools/trait/ToolResult/with_metadata.md)
- [display](../../../../../../../functions/src/llm/tools/skill/SkillSource/display.md)

# Called by

- [bash_runs_posix_in_the_requested_working_directory](../../../../../../../functions/src/llm/tools/bash/bash_runs_posix_in_the_requested_working_directory.md)
- [test_bash_simple_command](../../../../../../../functions/src/llm/tools/bash/test_bash_simple_command.md)
- [test_bash_with_exit_code](../../../../../../../functions/src/llm/tools/bash/test_bash_with_exit_code.md)
- [test_bash_invalid_command](../../../../../../../functions/src/llm/tools/bash/test_bash_invalid_command.md)
- [test_bash_timeout](../../../../../../../functions/src/llm/tools/bash/test_bash_timeout.md)
- [test_bash_accepts_directory_alias](../../../../../../../functions/src/llm/tools/bash/test_bash_accepts_directory_alias.md)
- [test_bash_timeout_field_overrides_context_default](../../../../../../../functions/src/llm/tools/bash/test_bash_timeout_field_overrides_context_default.md)
- [test_bash_is_background_notes_synchronous_fallback](../../../../../../../functions/src/llm/tools/bash/test_bash_is_background_notes_synchronous_fallback.md)