---
type: Rust Function
title: list_skills_discovers_legacy_flat_md_files
resource: src/llm/tools/skill.rs#L490-L506
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/project/write
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/list_skills
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/find
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn list_skills_discovers_legacy_flat_md_files()`

# Calls

- [write](../../../../../functions/src/memory/project/write.md)
- [list_skills](../../../../../functions/src/llm/tools/skill/list_skills.md)
- [find](../../../../../functions/src/memory/MemoryStore/find.md)