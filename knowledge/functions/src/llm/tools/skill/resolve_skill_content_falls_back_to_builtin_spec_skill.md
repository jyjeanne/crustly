---
type: Rust Function
title: resolve_skill_content_falls_back_to_builtin_spec_skill
resource: src/llm/tools/skill.rs#L588-L594
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/tools/skill/resolve_skill_content
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn resolve_skill_content_falls_back_to_builtin_spec_skill()`

# Calls

- [resolve_skill_content](../../../../../functions/src/llm/tools/skill/resolve_skill_content.md)
- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)