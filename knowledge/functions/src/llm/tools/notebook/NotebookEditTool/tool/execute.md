---
type: Rust Method
title: execute
resource: src/llm/tools/notebook.rs#L161-L335
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/tools/trait/ToolResult/error
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/sandbox/check_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/CodeKnowledge/exists
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/project/write
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/trait/ToolResult/success
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/skill/SkillSource/display
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/notebook/test_add_cell_within_working_directory_succeeds
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/notebook/test_path_outside_working_directory_is_denied
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>`

# Calls

- [error](../../../../../../../functions/src/llm/tools/trait/ToolResult/error.md)
- [check_path](../../../../../../../functions/src/llm/tools/sandbox/check_path.md)
- [exists](../../../../../../../functions/src/memory/code_knowledge/CodeKnowledge/exists.md)
- [write](../../../../../../../functions/src/memory/project/write.md)
- [contains](../../../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)
- [success](../../../../../../../functions/src/llm/tools/trait/ToolResult/success.md)
- [display](../../../../../../../functions/src/llm/tools/skill/SkillSource/display.md)

# Called by

- [test_add_cell_within_working_directory_succeeds](../../../../../../../functions/src/llm/tools/notebook/test_add_cell_within_working_directory_succeeds.md)
- [test_path_outside_working_directory_is_denied](../../../../../../../functions/src/llm/tools/notebook/test_path_outside_working_directory_is_denied.md)