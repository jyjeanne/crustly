---
type: Rust Function
title: a_secret_never_reaches_the_database_file_or_its_wal
resource: tests/memory_acceptance_test.rs#L205-L279
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/tests/memory_acceptance_test/open
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/register_project
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/forget
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/tool_round/ToolRound/push
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/SkillSource/display
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_acceptance_test/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn a_secret_never_reaches_the_database_file_or_its_wal()`

# Calls

- [open](../../../functions/tests/memory_acceptance_test/open.md)
- [register_project](../../../functions/src/memory/MemoryStore/register_project.md)
- [forget](../../../functions/src/memory/MemoryStore/forget.md)
- [push](../../../functions/src/llm/agent/service/tool_round/ToolRound/push.md)
- [display](../../../functions/src/llm/tools/skill/SkillSource/display.md)
- [contains](../../../functions/tests/memory_acceptance_test/contains.md)