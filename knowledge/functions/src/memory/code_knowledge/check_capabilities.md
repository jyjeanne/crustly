---
type: Rust Function
title: check_capabilities
resource: src/memory/code_knowledge.rs#L657-L687
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/find
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/schema_check/schema
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/all
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/code_knowledge/CodeKnowledge/start
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/capabilities_need_json_and_the_file_relation
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn check_capabilities(tools: &[McpToolDef]) -> Result<()>`

# Calls

- [find](../../../../functions/src/memory/MemoryStore/find.md)
- [schema](../../../../functions/src/llm/tools/schema_check/schema.md)
- [contains](../../../../functions/src/memory/vector_index/VectorIndex/contains.md)
- [all](../../../../functions/src/memory/MemoryStore/all.md)

# Called by

- [start](../../../../functions/src/memory/code_knowledge/CodeKnowledge/start.md)
- [capabilities_need_json_and_the_file_relation](../../../../functions/src/memory/code_knowledge/capabilities_need_json_and_the_file_relation.md)