---
type: Rust Method
title: set_status
resource: src/memory/code_knowledge.rs#L298-L302
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/memory/code_knowledge/CodeKnowledge/start
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/CodeKnowledge/call_json
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn set_status(&self, status: Status)`

# Called by

- [start](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/start.md)
- [call_json](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/call_json.md)