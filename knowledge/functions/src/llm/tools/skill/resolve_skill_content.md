---
type: Rust Function
title: resolve_skill_content
resource: src/llm/tools/skill.rs#L198-L209
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
  - target: functions/src/llm/tools/skill/parse_skill_frontmatter_value
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/skill/resolve_skill_content_falls_back_to_builtin_review_skill
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/resolve_skill_content_prefers_project_file_over_builtin
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/resolve_skill_content_falls_back_to_builtin_spec_skill
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/resolve_skill_content_returns_none_for_unknown_skill
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/try_handle_slash_command
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(crate) fn resolve_skill_content(name: &str, cwd: &Path) -> Option<(String, Option<String>)>`

# Calls

- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)
- [resolve_skill](../../../../../functions/src/llm/tools/skill/resolve_skill.md)
- [parse_skill_frontmatter_value](../../../../../functions/src/llm/tools/skill/parse_skill_frontmatter_value.md)

# Called by

- [resolve_skill_content_falls_back_to_builtin_review_skill](../../../../../functions/src/llm/tools/skill/resolve_skill_content_falls_back_to_builtin_review_skill.md)
- [resolve_skill_content_prefers_project_file_over_builtin](../../../../../functions/src/llm/tools/skill/resolve_skill_content_prefers_project_file_over_builtin.md)
- [resolve_skill_content_falls_back_to_builtin_spec_skill](../../../../../functions/src/llm/tools/skill/resolve_skill_content_falls_back_to_builtin_spec_skill.md)
- [resolve_skill_content_returns_none_for_unknown_skill](../../../../../functions/src/llm/tools/skill/resolve_skill_content_returns_none_for_unknown_skill.md)
- [try_handle_slash_command](../../../../../functions/src/tui/app/App/try_handle_slash_command.md)