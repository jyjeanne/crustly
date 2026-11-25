---
type: Rust Function
title: lookup
resource: src/llm/tools/skill.rs#L34-L39
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/find
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/skill/resolve_skill
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/list_skills
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(super) fn lookup(name: &str) -> Option<&'static str>`

# Calls

- [find](../../../../../functions/src/memory/MemoryStore/find.md)

# Called by

- [resolve_skill](../../../../../functions/src/llm/tools/skill/resolve_skill.md)
- [list_skills](../../../../../functions/src/llm/tools/skill/list_skills.md)