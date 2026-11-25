---
type: Rust Function
title: a_concept_gone_from_a_newer_bundle_is_flagged_and_not_from_an_older_one
resource: tests/memory_code_knowledge_test.rs#L373-L403
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/tests/memory_code_knowledge_test/Scripted/remove
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/project/write
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/anchors/MemoryStore/drifted
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/anchors/MemoryStore/start_code_knowledge
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/anchors/MemoryStore/anchors
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/get
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn a_concept_gone_from_a_newer_bundle_is_flagged_and_not_from_an_older_one()`

# Calls

- [remove](../../../functions/tests/memory_code_knowledge_test/Scripted/remove.md)
- [write](../../../functions/src/memory/project/write.md)
- [drifted](../../../functions/src/memory/anchors/MemoryStore/drifted.md)
- [start_code_knowledge](../../../functions/src/memory/anchors/MemoryStore/start_code_knowledge.md)
- [anchors](../../../functions/src/memory/anchors/MemoryStore/anchors.md)
- [get](../../../functions/src/memory/MemoryStore/get.md)