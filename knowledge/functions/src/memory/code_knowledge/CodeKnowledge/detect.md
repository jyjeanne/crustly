---
type: Rust Method
title: detect
resource: src/memory/code_knowledge.rs#L240-L277
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/code_knowledge/bundle_dir
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/mcp/client/MCPClient/connect
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/code_knowledge/CodeKnowledge/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/cli/build_memory_store
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/detection_is_off_without_a_command_or_a_bundle
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_code_knowledge_test/real_okf_mcp_answers_in_json
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn detect(config: &MemoryCodeKnowledgeConfig, root: &Path) -> Detection`

# Calls

- [bundle_dir](../../../../../functions/src/memory/code_knowledge/bundle_dir.md)
- [connect](../../../../../functions/src/mcp/client/MCPClient/connect.md)
- [new](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/new.md)

# Called by

- [build_memory_store](../../../../../functions/src/cli/build_memory_store.md)
- [detection_is_off_without_a_command_or_a_bundle](../../../../../functions/src/memory/code_knowledge/detection_is_off_without_a_command_or_a_bundle.md)
- [real_okf_mcp_answers_in_json](../../../../../functions/tests/memory_code_knowledge_test/real_okf_mcp_answers_in_json.md)