---
type: Rust Function
title: apply_hunks
resource: src/llm/tools/apply_patch.rs#L235-L285
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/tools/apply_patch/find_subsequence
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/tool_round/ToolRound/push
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/apply_patch/ApplyPatchTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/apply_patch/apply_hunks_replaces_matched_context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/apply_patch/apply_hunks_second_hunk_searches_after_first
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/apply_patch/apply_hunks_errors_when_context_not_found
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn apply_hunks(original: &str, hunks: &[Hunk]) -> std::result::Result<String, String>`

# Calls

- [find_subsequence](../../../../../functions/src/llm/tools/apply_patch/find_subsequence.md)
- [push](../../../../../functions/src/llm/agent/service/tool_round/ToolRound/push.md)

# Called by

- [execute](../../../../../functions/src/llm/tools/apply_patch/ApplyPatchTool/tool/execute.md)
- [apply_hunks_replaces_matched_context](../../../../../functions/src/llm/tools/apply_patch/apply_hunks_replaces_matched_context.md)
- [apply_hunks_second_hunk_searches_after_first](../../../../../functions/src/llm/tools/apply_patch/apply_hunks_second_hunk_searches_after_first.md)
- [apply_hunks_errors_when_context_not_found](../../../../../functions/src/llm/tools/apply_patch/apply_hunks_errors_when_context_not_found.md)