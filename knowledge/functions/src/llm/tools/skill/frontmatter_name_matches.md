---
type: Rust Function
title: frontmatter_name_matches
resource: src/llm/tools/skill.rs#L272-L277
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/tools/skill/parse_skill_frontmatter_value
  called_by:
  - functions/src/llm/tools/skill/resolve_skill_path
---

# Signature

`fn frontmatter_name_matches(path: &Path, requested: &str) -> bool`

# Calls

- [parse_skill_frontmatter_value](../../../../../functions/src/llm/tools/skill/parse_skill_frontmatter_value.md)

# Called by

- [resolve_skill_path](../../../../../functions/src/llm/tools/skill/resolve_skill_path.md)