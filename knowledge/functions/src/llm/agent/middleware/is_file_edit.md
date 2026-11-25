---
type: Rust Function
title: is_file_edit
resource: src/llm/agent/middleware.rs#L434-L436
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/service/AgentService/log_progress
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(crate) fn is_file_edit(name: &str) -> bool`

# Calls

- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)

# Called by

- [log_progress](../../../../../functions/src/llm/agent/service/AgentService/log_progress.md)