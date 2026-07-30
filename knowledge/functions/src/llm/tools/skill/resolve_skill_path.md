---
type: Rust Function
title: resolve_skill_path
resource: src/llm/tools/skill.rs#L137-L174
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/tools/skill/skill_lookup_roots
  - functions/src/llm/tools/skill/frontmatter_name_matches
  called_by:
  - functions/src/llm/tools/skill/SkillTool/tool/execute
---

# Signature

`fn resolve_skill_path(name: &str, cwd: &Path) -> std::result::Result<PathBuf, String>`

# Calls

- [skill_lookup_roots](../../../../../functions/src/llm/tools/skill/skill_lookup_roots.md)
- [frontmatter_name_matches](../../../../../functions/src/llm/tools/skill/frontmatter_name_matches.md)

# Called by

- [execute](../../../../../functions/src/llm/tools/skill/SkillTool/tool/execute.md)