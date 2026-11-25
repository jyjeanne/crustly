---
type: Rust Function
title: resolve_existing_prefix
resource: src/llm/tools/sandbox.rs#L455-L480
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/tools/sandbox/normalize_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/tool_round/ToolRound/push
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/sandbox/PathBoundaryRule/check
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn resolve_existing_prefix(path: &Path) -> PathBuf`

# Calls

- [normalize_path](../../../../../functions/src/llm/tools/sandbox/normalize_path.md)
- [push](../../../../../functions/src/llm/agent/service/tool_round/ToolRound/push.md)

# Called by

- [check](../../../../../functions/src/llm/tools/sandbox/PathBoundaryRule/check.md)