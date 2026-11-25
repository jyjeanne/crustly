---
type: Rust Method
title: explore
resource: src/memory/code_knowledge.rs#L500-L513
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/code_knowledge/CodeKnowledge/call_json
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/code_knowledge/CodeKnowledge/neighbours
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/CodeKnowledge/location
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn explore(&self, id: &str) -> Option<std::result::Result<Value, String>>`

# Calls

- [call_json](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/call_json.md)

# Called by

- [neighbours](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/neighbours.md)
- [location](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/location.md)