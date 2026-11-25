---
type: Rust Function
title: recall_block_reaches_the_model_but_not_the_database
resource: src/llm/agent/service.rs#L4114-L4178
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/test_context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/session_for
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/memory_test_store
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/save
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/ScriptedProvider/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/AgentService/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/AgentService/with_working_directory
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/AgentService/with_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/send_message_with_tools
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/ScriptedProvider/requests
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/find
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/ServiceContext/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/text_reply
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/all
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn recall_block_reaches_the_model_but_not_the_database()`

# Calls

- [test_context](../../../../../functions/src/llm/agent/service/test_context.md)
- [session_for](../../../../../functions/src/llm/agent/service/session_for.md)
- [memory_test_store](../../../../../functions/src/llm/agent/service/memory_test_store.md)
- [save](../../../../../functions/src/memory/MemoryStore/save.md)
- [new](../../../../../functions/src/llm/agent/service/ScriptedProvider/new.md)
- [new](../../../../../functions/src/llm/agent/service/AgentService/new.md)
- [with_working_directory](../../../../../functions/src/llm/agent/service/AgentService/with_working_directory.md)
- [with_memory](../../../../../functions/src/llm/agent/service/AgentService/with_memory.md)
- [send_message_with_tools](../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools.md)
- [requests](../../../../../functions/src/llm/agent/service/ScriptedProvider/requests.md)
- [find](../../../../../functions/src/memory/MemoryStore/find.md)
- [pool](../../../../../functions/src/services/ServiceContext/pool.md)
- [text_reply](../../../../../functions/src/llm/agent/service/text_reply.md)
- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)
- [all](../../../../../functions/src/memory/MemoryStore/all.md)