---
type: Rust Function
title: validate_directory_path
resource: src/llm/tools/error.rs#L154-L183
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/tools/error/validate_path_safety
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/CodeKnowledge/exists
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/SkillSource/display
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn validate_directory_path( requested_path: &str, working_directory: &std::path::Path, ) -> std::result::Result<std::path::PathBuf, String>`

# Calls

- [validate_path_safety](../../../../../functions/src/llm/tools/error/validate_path_safety.md)
- [exists](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/exists.md)
- [display](../../../../../functions/src/llm/tools/skill/SkillSource/display.md)