---
type: Rust Function
title: real_okf_mcp_answers_in_json
resource: tests/memory_code_knowledge_test.rs#L464-L498
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/project/write
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/CodeKnowledge/detect
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/identifiers
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/CodeKnowledge/status_line
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/CodeKnowledge/location
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/CodeKnowledge/file_concepts
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/CodeKnowledge/exists
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn real_okf_mcp_answers_in_json()`

# Calls

- [write](../../../functions/src/memory/project/write.md)
- [detect](../../../functions/src/memory/code_knowledge/CodeKnowledge/detect.md)
- [identifiers](../../../functions/src/memory/code_knowledge/identifiers.md)
- [status_line](../../../functions/src/memory/code_knowledge/CodeKnowledge/status_line.md)
- [location](../../../functions/src/memory/code_knowledge/CodeKnowledge/location.md)
- [file_concepts](../../../functions/src/memory/code_knowledge/CodeKnowledge/file_concepts.md)
- [exists](../../../functions/src/memory/code_knowledge/CodeKnowledge/exists.md)