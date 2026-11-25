---
type: Rust Function
title: resolve_skill
resource: src/llm/tools/skill.rs#L185-L190
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/tools/skill/resolve_skill_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/lookup
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/skill/SkillTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/resolve_skill_content
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn resolve_skill(name: &str, cwd: &Path) -> Option<SkillSource>`

# Calls

- [resolve_skill_path](../../../../../functions/src/llm/tools/skill/resolve_skill_path.md)
- [lookup](../../../../../functions/src/llm/tools/skill/lookup.md)

# Called by

- [execute](../../../../../functions/src/llm/tools/skill/SkillTool/tool/execute.md)
- [resolve_skill_content](../../../../../functions/src/llm/tools/skill/resolve_skill_content.md)