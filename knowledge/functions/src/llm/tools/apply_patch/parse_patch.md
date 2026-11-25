---
type: Rust Function
title: parse_patch
resource: src/llm/tools/apply_patch.rs#L92-L215
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/tool_round/ToolRound/push
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/apply_patch/ApplyPatchTool/tool/validate_input
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/apply_patch/ApplyPatchTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/apply_patch/parse_rejects_missing_begin_marker
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/apply_patch/parse_rejects_missing_end_marker
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/apply_patch/parse_add_file_collects_plus_prefixed_lines
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/apply_patch/parse_multiple_file_ops_in_one_patch
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/apply_patch/parse_update_with_move_to
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn parse_patch(text: &str) -> std::result::Result<Vec<FileOp>, String>`

# Calls

- [push](../../../../../functions/src/llm/agent/service/tool_round/ToolRound/push.md)

# Called by

- [validate_input](../../../../../functions/src/llm/tools/apply_patch/ApplyPatchTool/tool/validate_input.md)
- [execute](../../../../../functions/src/llm/tools/apply_patch/ApplyPatchTool/tool/execute.md)
- [parse_rejects_missing_begin_marker](../../../../../functions/src/llm/tools/apply_patch/parse_rejects_missing_begin_marker.md)
- [parse_rejects_missing_end_marker](../../../../../functions/src/llm/tools/apply_patch/parse_rejects_missing_end_marker.md)
- [parse_add_file_collects_plus_prefixed_lines](../../../../../functions/src/llm/tools/apply_patch/parse_add_file_collects_plus_prefixed_lines.md)
- [parse_multiple_file_ops_in_one_patch](../../../../../functions/src/llm/tools/apply_patch/parse_multiple_file_ops_in_one_patch.md)
- [parse_update_with_move_to](../../../../../functions/src/llm/tools/apply_patch/parse_update_with_move_to.md)