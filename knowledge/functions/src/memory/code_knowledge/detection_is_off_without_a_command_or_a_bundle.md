---
type: Rust Function
title: detection_is_off_without_a_command_or_a_bundle
resource: src/memory/code_knowledge.rs#L819-L842
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/config/MemoryCodeKnowledgeConfig/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/project/write
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/CodeKnowledge/detect
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn detection_is_off_without_a_command_or_a_bundle()`

# Calls

- [default](../../../../functions/src/config/MemoryCodeKnowledgeConfig/default/default.md)
- [write](../../../../functions/src/memory/project/write.md)
- [detect](../../../../functions/src/memory/code_knowledge/CodeKnowledge/detect.md)
- [contains](../../../../functions/src/memory/vector_index/VectorIndex/contains.md)