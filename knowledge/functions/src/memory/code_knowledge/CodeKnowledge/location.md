---
type: Rust Method
title: location
resource: src/memory/code_knowledge.rs#L538-L549
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/code_knowledge/CodeKnowledge/explore
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/cmd_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/anchors/MemoryStore/anchor_locations
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_code_knowledge_test/real_okf_mcp_answers_in_json
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn location(&self, id: &str) -> Option<String>`

# Calls

- [explore](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/explore.md)

# Called by

- [cmd_memory](../../../../../functions/src/cli/cmd_memory.md)
- [anchor_locations](../../../../../functions/src/memory/anchors/MemoryStore/anchor_locations.md)
- [real_okf_mcp_answers_in_json](../../../../../functions/tests/memory_code_knowledge_test/real_okf_mcp_answers_in_json.md)