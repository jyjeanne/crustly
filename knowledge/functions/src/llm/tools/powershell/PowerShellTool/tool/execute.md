---
type: Rust Method
title: execute
resource: src/llm/tools/powershell.rs#L239-L364
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/tools/powershell/is_read_only_powershell
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/trait/ToolResult/error
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/trait/ToolResult/success
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/trait/ToolResult/with_metadata
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/error/CrustlyError/code
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/SkillSource/display
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/powershell/execute_blocks_dangerous_command_in_read_only_mode
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/powershell/execute_allows_read_only_command_in_plan_mode
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>`

# Calls

- [is_read_only_powershell](../../../../../../../functions/src/llm/tools/powershell/is_read_only_powershell.md)
- [error](../../../../../../../functions/src/llm/tools/trait/ToolResult/error.md)
- [success](../../../../../../../functions/src/llm/tools/trait/ToolResult/success.md)
- [with_metadata](../../../../../../../functions/src/llm/tools/trait/ToolResult/with_metadata.md)
- [code](../../../../../../../functions/src/error/CrustlyError/code.md)
- [display](../../../../../../../functions/src/llm/tools/skill/SkillSource/display.md)

# Called by

- [execute_blocks_dangerous_command_in_read_only_mode](../../../../../../../functions/src/llm/tools/powershell/execute_blocks_dangerous_command_in_read_only_mode.md)
- [execute_allows_read_only_command_in_plan_mode](../../../../../../../functions/src/llm/tools/powershell/execute_allows_read_only_command_in_plan_mode.md)