---
type: Rust Method
title: evaluate
resource: src/llm/tools/sandbox.rs#L104-L119
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/tools/sandbox/normalize_path
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/sandbox/deny_path_prefix_blocks_matching_path
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/sandbox/deny_path_prefix_allows_unrelated_path
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn evaluate(&self, _tool_name: &str, inputs: &Value) -> PolicyDecision`

# Calls

- [normalize_path](../../../../../../../functions/src/llm/tools/sandbox/normalize_path.md)

# Called by

- [deny_path_prefix_blocks_matching_path](../../../../../../../functions/src/llm/tools/sandbox/deny_path_prefix_blocks_matching_path.md)
- [deny_path_prefix_allows_unrelated_path](../../../../../../../functions/src/llm/tools/sandbox/deny_path_prefix_allows_unrelated_path.md)