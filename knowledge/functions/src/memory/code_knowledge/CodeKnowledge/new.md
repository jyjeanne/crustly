---
type: Rust Method
title: new
resource: src/memory/code_knowledge.rs#L281-L289
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/memory/code_knowledge/CodeKnowledge/detect
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_code_knowledge_test/setup
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn new(bundle: PathBuf, timeout: Duration, connector: Connector) -> Self`

# Called by

- [detect](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/detect.md)
- [setup](../../../../../functions/tests/memory_code_knowledge_test/setup.md)