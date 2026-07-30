---
type: Rust Method
title: execute
resource: src/llm/tools/skill.rs#L95-L133
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/is_empty
  - functions/src/llm/tools/skill/resolve_skill_path
  - functions/src/llm/tools/skill/parse_skill_frontmatter_value
  - functions/src/llm/tools/trait/ToolResult/with_metadata
---

# Signature

`async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>`

# Calls

- [is_empty](../../../../../../../functions/src/config/secrets/SecretString/is_empty.md)
- [resolve_skill_path](../../../../../../../functions/src/llm/tools/skill/resolve_skill_path.md)
- [parse_skill_frontmatter_value](../../../../../../../functions/src/llm/tools/skill/parse_skill_frontmatter_value.md)
- [with_metadata](../../../../../../../functions/src/llm/tools/trait/ToolResult/with_metadata.md)