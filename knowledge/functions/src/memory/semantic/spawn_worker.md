---
type: Rust Function
title: spawn_worker
resource: src/memory/semantic.rs#L158-L200
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/semantic/backfill
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/tool_round/ToolRound/push
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/get
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/semantic/embed_and_store
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/MemoryStore/attach_embedder
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn spawn_worker( repo: MemoryRepository, embedder: Arc<dyn Embedder>, runtime: EmbedRuntime, generation: Arc<AtomicU64>, mut queue: mpsc::Receiver<Uuid>, )`

# Calls

- [backfill](../../../../functions/src/memory/semantic/backfill.md)
- [push](../../../../functions/src/llm/agent/service/tool_round/ToolRound/push.md)
- [get](../../../../functions/src/db/repository/memory/MemoryRepository/get.md)
- [embed_and_store](../../../../functions/src/memory/semantic/embed_and_store.md)

# Called by

- [attach_embedder](../../../../functions/src/memory/MemoryStore/attach_embedder.md)