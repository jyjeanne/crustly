---
type: Rust Function
title: notes_for_src
resource: src/llm/agent/service/run.rs#L714-L729
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/run/AgentRun/path_memory_text
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/service/run/a_path_memory_cannot_close_the_notes_block
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/run/an_expired_path_memory_is_not_sent
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn notes_for_src( store: &Arc<crate::memory::MemoryStore>, dir: &tempfile::TempDir, ) -> Option<String>`

# Calls

- [path_memory_text](../../../../../../functions/src/llm/agent/service/run/AgentRun/path_memory_text.md)

# Called by

- [a_path_memory_cannot_close_the_notes_block](../../../../../../functions/src/llm/agent/service/run/a_path_memory_cannot_close_the_notes_block.md)
- [an_expired_path_memory_is_not_sent](../../../../../../functions/src/llm/agent/service/run/an_expired_path_memory_is_not_sent.md)