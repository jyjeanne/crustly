---
type: Rust Method
title: check
resource: src/llm/tools/sandbox.rs#L129-L186
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/tools/sandbox/normalize_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/sandbox/strip_verbatim_prefix
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/sandbox/resolve_existing_prefix
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/SkillSource/display
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/sandbox/PathBoundaryRule/permissionpolicy/evaluate
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn check(&self, raw: &str) -> PolicyDecision`

# Calls

- [normalize_path](../../../../../../functions/src/llm/tools/sandbox/normalize_path.md)
- [strip_verbatim_prefix](../../../../../../functions/src/llm/tools/sandbox/strip_verbatim_prefix.md)
- [resolve_existing_prefix](../../../../../../functions/src/llm/tools/sandbox/resolve_existing_prefix.md)
- [display](../../../../../../functions/src/llm/tools/skill/SkillSource/display.md)

# Called by

- [evaluate](../../../../../../functions/src/llm/tools/sandbox/PathBoundaryRule/permissionpolicy/evaluate.md)