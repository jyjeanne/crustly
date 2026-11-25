---
type: Rust Method
title: trigger
resource: src/memory/extraction.rs#L51-L75
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/extraction/MemoryStore/extract
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/remove
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/service/AgentService/memory_block
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/end_session_in_background
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_extraction_test/the_background_trigger_runs_once_per_session_at_a_time
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn trigger( self: &Arc<Self>, session: Uuid, project: ProjectIdentity, session_end: bool, force: bool, )`

# Calls

- [extract](../../../../../functions/src/memory/extraction/MemoryStore/extract.md)
- [remove](../../../../../functions/src/memory/vector_index/VectorIndex/remove.md)

# Called by

- [memory_block](../../../../../functions/src/llm/agent/service/AgentService/memory_block.md)
- [end_session_in_background](../../../../../functions/src/tui/app/App/end_session_in_background.md)
- [the_background_trigger_runs_once_per_session_at_a_time](../../../../../functions/tests/memory_extraction_test/the_background_trigger_runs_once_per_session_at_a_time.md)