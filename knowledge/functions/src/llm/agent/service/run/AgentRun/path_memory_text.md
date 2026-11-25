---
type: Rust Method
title: path_memory_text
resource: src/llm/agent/service/run.rs#L248-L301
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/project/identify
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/memories_for_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/context/token_count
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/mark_shown
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/recall/clean
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/service/run/AgentRun/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/run/path_notes_cut_by_the_cap_are_sent_in_a_later_step
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/run/notes_for_src
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn path_memory_text( store: Option<std::sync::Arc<crate::memory::MemoryStore>>, working_directory: std::path::PathBuf, session: uuid::Uuid, records: Vec<crate::llm::agent::middleware::ToolCallRecord>, ) -> Option<String>`

# Calls

- [identify](../../../../../../../functions/src/memory/project/identify.md)
- [memories_for_path](../../../../../../../functions/src/memory/MemoryStore/memories_for_path.md)
- [token_count](../../../../../../../functions/src/llm/agent/context/token_count.md)
- [mark_shown](../../../../../../../functions/src/memory/MemoryStore/mark_shown.md)
- [clean](../../../../../../../functions/src/memory/recall/clean.md)

# Called by

- [run](../../../../../../../functions/src/llm/agent/service/run/AgentRun/run.md)
- [path_notes_cut_by_the_cap_are_sent_in_a_later_step](../../../../../../../functions/src/llm/agent/service/run/path_notes_cut_by_the_cap_are_sent_in_a_later_step.md)
- [notes_for_src](../../../../../../../functions/src/llm/agent/service/run/notes_for_src.md)