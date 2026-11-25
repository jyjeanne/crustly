---
type: Rust Method
title: new
resource: src/llm/tools/sandbox.rs#L96-L100
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/config/SecurityConfig/to_policy
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/sandbox/deny_path_prefix_blocks_matching_path
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/sandbox/deny_path_prefix_allows_unrelated_path
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn new(raw: &str) -> Self`

# Called by

- [to_policy](../../../../../../functions/src/config/SecurityConfig/to_policy.md)
- [deny_path_prefix_blocks_matching_path](../../../../../../functions/src/llm/tools/sandbox/deny_path_prefix_blocks_matching_path.md)
- [deny_path_prefix_allows_unrelated_path](../../../../../../functions/src/llm/tools/sandbox/deny_path_prefix_allows_unrelated_path.md)