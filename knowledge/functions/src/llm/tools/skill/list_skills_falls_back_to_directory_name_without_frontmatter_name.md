---
type: Rust Function
title: list_skills_falls_back_to_directory_name_without_frontmatter_name
resource: src/llm/tools/skill.rs#L475-L487
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
---

# Signature

`fn list_skills_falls_back_to_directory_name_without_frontmatter_name()`

# Calls

- [write](../../../../../functions/src/memory/project/write.md)
- [list_skills](../../../../../functions/src/llm/tools/skill/list_skills.md)