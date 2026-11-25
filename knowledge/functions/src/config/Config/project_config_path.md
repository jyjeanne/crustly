---
type: Rust Method
title: project_config_path
resource: src/config/mod.rs#L1485-L1496
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/code_knowledge/CodeKnowledge/exists
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/config/Config/load
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn project_config_path() -> Option<PathBuf>`

# Calls

- [exists](../../../../functions/src/memory/code_knowledge/CodeKnowledge/exists.md)

# Called by

- [load](../../../../functions/src/config/Config/load.md)