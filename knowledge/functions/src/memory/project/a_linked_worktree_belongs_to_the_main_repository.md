---
type: Rust Function
title: a_linked_worktree_belongs_to_the_main_repository
resource: src/memory/project.rs#L158-L175
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
  - target: functions/src/memory/project/identify
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/SkillSource/display
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn a_linked_worktree_belongs_to_the_main_repository()`

# Calls

- [write](../../../../functions/src/memory/project/write.md)
- [identify](../../../../functions/src/memory/project/identify.md)
- [display](../../../../functions/src/llm/tools/skill/SkillSource/display.md)