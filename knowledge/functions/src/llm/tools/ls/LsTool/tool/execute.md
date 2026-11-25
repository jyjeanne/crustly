---
type: Rust Method
title: execute
resource: src/llm/tools/ls.rs#L86-L134
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/tools/sandbox/check_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/trait/ToolResult/error
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/code_knowledge/CodeKnowledge/exists
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/ls/LsTool/list_recursive
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/ls/LsTool/list_directory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/trait/ToolResult/success
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/skill/SkillSource/display
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>`

# Calls

- [check_path](../../../../../../../functions/src/llm/tools/sandbox/check_path.md)
- [error](../../../../../../../functions/src/llm/tools/trait/ToolResult/error.md)
- [exists](../../../../../../../functions/src/memory/code_knowledge/CodeKnowledge/exists.md)
- [list_recursive](../../../../../../../functions/src/llm/tools/ls/LsTool/list_recursive.md)
- [list_directory](../../../../../../../functions/src/llm/tools/ls/LsTool/list_directory.md)
- [success](../../../../../../../functions/src/llm/tools/trait/ToolResult/success.md)
- [display](../../../../../../../functions/src/llm/tools/skill/SkillSource/display.md)