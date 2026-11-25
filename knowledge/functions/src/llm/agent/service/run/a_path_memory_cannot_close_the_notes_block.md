---
type: Rust Function
title: a_path_memory_cannot_close_the_notes_block
resource: src/llm/agent/service/run.rs#L739-L745
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/run/store_with_path_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/run/notes_for_src
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn a_path_memory_cannot_close_the_notes_block()`

# Calls

- [store_with_path_memory](../../../../../../functions/src/llm/agent/service/run/store_with_path_memory.md)
- [notes_for_src](../../../../../../functions/src/llm/agent/service/run/notes_for_src.md)