---
type: Rust Function
title: an_expired_path_memory_is_not_sent
resource: src/llm/agent/service/run.rs#L732-L736
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

`async fn an_expired_path_memory_is_not_sent()`

# Calls

- [store_with_path_memory](../../../../../../functions/src/llm/agent/service/run/store_with_path_memory.md)
- [notes_for_src](../../../../../../functions/src/llm/agent/service/run/notes_for_src.md)