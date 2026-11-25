---
type: Rust Method
title: mark_shown
resource: src/memory/mod.rs#L846-L852
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/agent/service/run/AgentRun/path_memory_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_code_knowledge_test/reading_the_file_that_defines_the_concept_brings_the_memory_once
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_paths_test/a_path_memory_is_shown_once_per_session
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn mark_shown(&self, session: Uuid, ids: &[Uuid])`

# Called by

- [path_memory_text](../../../../functions/src/llm/agent/service/run/AgentRun/path_memory_text.md)
- [reading_the_file_that_defines_the_concept_brings_the_memory_once](../../../../functions/tests/memory_code_knowledge_test/reading_the_file_that_defines_the_concept_brings_the_memory_once.md)
- [a_path_memory_is_shown_once_per_session](../../../../functions/tests/memory_paths_test/a_path_memory_is_shown_once_per_session.md)