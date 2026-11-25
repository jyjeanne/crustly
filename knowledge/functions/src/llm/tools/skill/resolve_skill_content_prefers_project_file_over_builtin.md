---
type: Rust Function
title: resolve_skill_content_prefers_project_file_over_builtin
resource: src/llm/tools/skill.rs#L559-L572
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
  - target: functions/src/llm/tools/skill/resolve_skill_content
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn resolve_skill_content_prefers_project_file_over_builtin()`

# Calls

- [write](../../../../../functions/src/memory/project/write.md)
- [resolve_skill_content](../../../../../functions/src/llm/tools/skill/resolve_skill_content.md)
- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)