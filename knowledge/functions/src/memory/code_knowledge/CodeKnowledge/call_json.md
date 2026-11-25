---
type: Rust Method
title: call_json
resource: src/memory/code_knowledge.rs#L390-L425
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/code_knowledge/CodeKnowledge/status
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/CodeKnowledge/set_status
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/code_knowledge/CodeKnowledge/resolve
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/CodeKnowledge/explore
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/CodeKnowledge/file_concepts
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/CodeKnowledge/exists
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/CodeKnowledge/closest
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn call_json( &self, tool: &str, arguments: Value, schema: &str, ) -> Option<std::result::Result<Value, String>>`

# Calls

- [status](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/status.md)
- [set_status](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/set_status.md)

# Called by

- [resolve](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/resolve.md)
- [explore](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/explore.md)
- [file_concepts](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/file_concepts.md)
- [exists](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/exists.md)
- [closest](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/closest.md)