---
type: Rust Function
title: bundle_dir
resource: src/memory/code_knowledge.rs#L226-L235
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/connect_configured_mcp_servers
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/CodeKnowledge/detect
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/a_bundle_is_found_through_okf_toml_or_knowledge
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn bundle_dir(root: &Path) -> Option<PathBuf>`

# Calls

- [contains](../../../../functions/src/memory/vector_index/VectorIndex/contains.md)

# Called by

- [connect_configured_mcp_servers](../../../../functions/src/cli/connect_configured_mcp_servers.md)
- [detect](../../../../functions/src/memory/code_knowledge/CodeKnowledge/detect.md)
- [a_bundle_is_found_through_okf_toml_or_knowledge](../../../../functions/src/memory/code_knowledge/a_bundle_is_found_through_okf_toml_or_knowledge.md)