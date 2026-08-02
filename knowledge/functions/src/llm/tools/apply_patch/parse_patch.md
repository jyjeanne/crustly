---
type: Rust Function
title: parse_patch
resource: src/llm/tools/apply_patch.rs#L92-L215
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/len
  - functions/src/config/secrets/SecretString/is_empty
  called_by:
  - functions/src/llm/tools/apply_patch/ApplyPatchTool/tool/validate_input
  - functions/src/llm/tools/apply_patch/ApplyPatchTool/tool/execute
  - functions/src/llm/tools/apply_patch/parse_rejects_missing_begin_marker
  - functions/src/llm/tools/apply_patch/parse_rejects_missing_end_marker
  - functions/src/llm/tools/apply_patch/parse_add_file_collects_plus_prefixed_lines
  - functions/src/llm/tools/apply_patch/parse_multiple_file_ops_in_one_patch
  - functions/src/llm/tools/apply_patch/parse_update_with_move_to
---

# Signature

`fn parse_patch(text: &str) -> std::result::Result<Vec<FileOp>, String>`

# Calls

- [len](../../../../../functions/src/config/secrets/SecretString/len.md)
- [is_empty](../../../../../functions/src/config/secrets/SecretString/is_empty.md)

# Called by

- [validate_input](../../../../../functions/src/llm/tools/apply_patch/ApplyPatchTool/tool/validate_input.md)
- [execute](../../../../../functions/src/llm/tools/apply_patch/ApplyPatchTool/tool/execute.md)
- [parse_rejects_missing_begin_marker](../../../../../functions/src/llm/tools/apply_patch/parse_rejects_missing_begin_marker.md)
- [parse_rejects_missing_end_marker](../../../../../functions/src/llm/tools/apply_patch/parse_rejects_missing_end_marker.md)
- [parse_add_file_collects_plus_prefixed_lines](../../../../../functions/src/llm/tools/apply_patch/parse_add_file_collects_plus_prefixed_lines.md)
- [parse_multiple_file_ops_in_one_patch](../../../../../functions/src/llm/tools/apply_patch/parse_multiple_file_ops_in_one_patch.md)
- [parse_update_with_move_to](../../../../../functions/src/llm/tools/apply_patch/parse_update_with_move_to.md)