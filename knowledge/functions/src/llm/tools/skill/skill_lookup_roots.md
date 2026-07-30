---
type: Rust Function
title: skill_lookup_roots
resource: src/llm/tools/skill.rs#L177-L203
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/tools/skill/push_if_dir
  called_by:
  - functions/src/llm/tools/skill/resolve_skill_path
  - functions/src/llm/tools/skill/list_skills
---

# Signature

`fn skill_lookup_roots(cwd: &Path) -> Vec<PathBuf>`

# Calls

- [push_if_dir](../../../../../functions/src/llm/tools/skill/push_if_dir.md)

# Called by

- [resolve_skill_path](../../../../../functions/src/llm/tools/skill/resolve_skill_path.md)
- [list_skills](../../../../../functions/src/llm/tools/skill/list_skills.md)