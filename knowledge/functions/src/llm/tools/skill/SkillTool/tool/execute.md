---
type: Rust Method
title: execute
resource: src/llm/tools/skill.rs#L120-L162
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/resolve_skill
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/SkillSource/display
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/parse_skill_frontmatter_value
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/trait/ToolResult/success
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/trait/ToolResult/with_metadata
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>`

# Calls

- [contains](../../../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)
- [resolve_skill](../../../../../../../functions/src/llm/tools/skill/resolve_skill.md)
- [display](../../../../../../../functions/src/llm/tools/skill/SkillSource/display.md)
- [parse_skill_frontmatter_value](../../../../../../../functions/src/llm/tools/skill/parse_skill_frontmatter_value.md)
- [success](../../../../../../../functions/src/llm/tools/trait/ToolResult/success.md)
- [with_metadata](../../../../../../../functions/src/llm/tools/trait/ToolResult/with_metadata.md)