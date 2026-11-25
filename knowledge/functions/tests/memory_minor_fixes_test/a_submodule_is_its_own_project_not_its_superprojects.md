---
type: Rust Function
title: a_submodule_is_its_own_project_not_its_superprojects
resource: tests/memory_minor_fixes_test.rs#L79-L102
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

`fn a_submodule_is_its_own_project_not_its_superprojects()`

# Calls

- [write](../../../functions/src/memory/project/write.md)
- [identify](../../../functions/src/memory/project/identify.md)
- [display](../../../functions/src/llm/tools/skill/SkillSource/display.md)