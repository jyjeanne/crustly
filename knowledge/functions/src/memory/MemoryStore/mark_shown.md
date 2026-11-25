---
type: Rust Method
title: mark_shown
resource: src/memory/mod.rs#L814-L820
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/agent/service/run/AgentRun/path_memory_text
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
- [a_path_memory_is_shown_once_per_session](../../../../functions/tests/memory_paths_test/a_path_memory_is_shown_once_per_session.md)