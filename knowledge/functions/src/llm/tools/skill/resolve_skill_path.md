---
type: Rust Function
title: resolve_skill_path
resource: src/llm/tools/skill.rs#L212-L249
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/tools/skill/skill_lookup_roots
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/frontmatter_name_matches
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/skill/resolve_skill
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn resolve_skill_path(name: &str, cwd: &Path) -> std::result::Result<PathBuf, String>`

# Calls

- [skill_lookup_roots](../../../../../functions/src/llm/tools/skill/skill_lookup_roots.md)
- [frontmatter_name_matches](../../../../../functions/src/llm/tools/skill/frontmatter_name_matches.md)

# Called by

- [resolve_skill](../../../../../functions/src/llm/tools/skill/resolve_skill.md)