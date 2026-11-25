---
type: Rust Function
title: push_if_dir
resource: src/llm/tools/skill.rs#L280-L284
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/tools/skill/skill_lookup_roots
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn push_if_dir(roots: &mut Vec<PathBuf>, path: PathBuf)`

# Called by

- [skill_lookup_roots](../../../../../functions/src/llm/tools/skill/skill_lookup_roots.md)