---
type: Rust Function
title: list_skills_is_sorted_alphabetically_case_insensitive
resource: src/llm/tools/skill.rs#L509-L525
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/project/write
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/list_skills
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn list_skills_is_sorted_alphabetically_case_insensitive()`

# Calls

- [write](../../../../../functions/src/memory/project/write.md)
- [list_skills](../../../../../functions/src/llm/tools/skill/list_skills.md)
- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)