---
type: Rust Function
title: memory_test_store
resource: src/llm/agent/service.rs#L4092-L4097
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/ServiceContext/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/MemoryConfig/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/llm/agent/service/recall_block_reaches_the_model_but_not_the_database
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/a_refused_forget_deletes_nothing_and_the_prompt_shows_the_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/a_refused_global_save_stores_nothing
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn memory_test_store(context: &ServiceContext) -> Arc<crate::memory::MemoryStore>`

# Calls

- [new](../../../../../functions/src/memory/MemoryStore/new.md)
- [pool](../../../../../functions/src/services/ServiceContext/pool.md)
- [default](../../../../../functions/src/config/MemoryConfig/default/default.md)

# Called by

- [recall_block_reaches_the_model_but_not_the_database](../../../../../functions/src/llm/agent/service/recall_block_reaches_the_model_but_not_the_database.md)
- [a_refused_forget_deletes_nothing_and_the_prompt_shows_the_memory](../../../../../functions/src/llm/agent/service/a_refused_forget_deletes_nothing_and_the_prompt_shows_the_memory.md)
- [a_refused_global_save_stores_nothing](../../../../../functions/src/llm/agent/service/a_refused_global_save_stores_nothing.md)