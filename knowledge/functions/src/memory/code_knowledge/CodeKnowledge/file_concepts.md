---
type: Rust Method
title: file_concepts
resource: src/memory/code_knowledge.rs#L516-L535
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/code_knowledge/CodeKnowledge/call_json
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/anchors/MemoryStore/anchored_in_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_code_knowledge_test/real_okf_mcp_answers_in_json
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn file_concepts(&self, path: &str) -> Vec<String>`

# Calls

- [call_json](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/call_json.md)

# Called by

- [anchored_in_file](../../../../../functions/src/memory/anchors/MemoryStore/anchored_in_file.md)
- [real_okf_mcp_answers_in_json](../../../../../functions/tests/memory_code_knowledge_test/real_okf_mcp_answers_in_json.md)