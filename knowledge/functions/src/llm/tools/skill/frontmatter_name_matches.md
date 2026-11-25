---
type: Rust Function
title: frontmatter_name_matches
resource: src/llm/tools/skill.rs#L360-L365
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/tools/skill/parse_skill_frontmatter_value
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/skill/resolve_skill_path
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn frontmatter_name_matches(path: &Path, requested: &str) -> bool`

# Calls

- [parse_skill_frontmatter_value](../../../../../functions/src/llm/tools/skill/parse_skill_frontmatter_value.md)

# Called by

- [resolve_skill_path](../../../../../functions/src/llm/tools/skill/resolve_skill_path.md)