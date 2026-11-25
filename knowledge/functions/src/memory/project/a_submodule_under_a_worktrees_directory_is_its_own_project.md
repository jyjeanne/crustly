---
type: Rust Function
title: a_submodule_under_a_worktrees_directory_is_its_own_project
resource: src/memory/project.rs#L198-L212
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
  - target: functions/src/memory/project/identify
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/SkillSource/display
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn a_submodule_under_a_worktrees_directory_is_its_own_project()`

# Calls

- [write](../../../../functions/src/memory/project/write.md)
- [identify](../../../../functions/src/memory/project/identify.md)
- [display](../../../../functions/src/llm/tools/skill/SkillSource/display.md)