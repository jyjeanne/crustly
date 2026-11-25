---
type: Rust Function
title: skill_lookup_roots
resource: src/llm/tools/skill.rs#L252-L278
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/tools/skill/push_if_dir
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/skill/resolve_skill_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/list_skills
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn skill_lookup_roots(cwd: &Path) -> Vec<PathBuf>`

# Calls

- [push_if_dir](../../../../../functions/src/llm/tools/skill/push_if_dir.md)

# Called by

- [resolve_skill_path](../../../../../functions/src/llm/tools/skill/resolve_skill_path.md)
- [list_skills](../../../../../functions/src/llm/tools/skill/list_skills.md)