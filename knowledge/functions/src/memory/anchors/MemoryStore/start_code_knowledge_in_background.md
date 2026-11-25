---
type: Rust Method
title: start_code_knowledge_in_background
resource: src/memory/anchors.rs#L73-L77
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/code_knowledge/CodeKnowledge/spawn_start
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/build_memory_store
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn start_code_knowledge_in_background(&self)`

# Calls

- [spawn_start](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/spawn_start.md)

# Called by

- [build_memory_store](../../../../../functions/src/cli/build_memory_store.md)