---
type: Rust Method
title: status_line
resource: src/memory/code_knowledge.rs#L310-L317
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/code_knowledge/CodeKnowledge/status
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/CodeKnowledge/bundle_shown
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/anchors/MemoryStore/code_knowledge_status
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_code_knowledge_test/real_okf_mcp_answers_in_json
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn status_line(&self) -> String`

# Calls

- [status](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/status.md)
- [bundle_shown](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/bundle_shown.md)

# Called by

- [code_knowledge_status](../../../../../functions/src/memory/anchors/MemoryStore/code_knowledge_status.md)
- [real_okf_mcp_answers_in_json](../../../../../functions/tests/memory_code_knowledge_test/real_okf_mcp_answers_in_json.md)