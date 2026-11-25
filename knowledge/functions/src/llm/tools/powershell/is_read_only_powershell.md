---
type: Rust Function
title: is_read_only_powershell
resource: src/llm/tools/powershell.rs#L128-L134
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/powershell/PowerShellTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/powershell/read_only_allows_get_content
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/powershell/read_only_allows_get_childitem
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/powershell/read_only_allows_select_string
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/powershell/read_only_blocks_remove_item
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/powershell/read_only_blocks_invoke_expression
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/powershell/read_only_blocks_pipe_to_out_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/powershell/read_only_blocks_net_method_call
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/powershell/read_only_blocks_iex_without_space
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/powershell/read_only_allows_gt_in_string_argument
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/powershell/read_only_blocks_redirection_with_space
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/powershell/read_only_blocks_append_no_spaces
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn is_read_only_powershell(command: &str) -> bool`

# Calls

- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)

# Called by

- [execute](../../../../../functions/src/llm/tools/powershell/PowerShellTool/tool/execute.md)
- [read_only_allows_get_content](../../../../../functions/src/llm/tools/powershell/read_only_allows_get_content.md)
- [read_only_allows_get_childitem](../../../../../functions/src/llm/tools/powershell/read_only_allows_get_childitem.md)
- [read_only_allows_select_string](../../../../../functions/src/llm/tools/powershell/read_only_allows_select_string.md)
- [read_only_blocks_remove_item](../../../../../functions/src/llm/tools/powershell/read_only_blocks_remove_item.md)
- [read_only_blocks_invoke_expression](../../../../../functions/src/llm/tools/powershell/read_only_blocks_invoke_expression.md)
- [read_only_blocks_pipe_to_out_file](../../../../../functions/src/llm/tools/powershell/read_only_blocks_pipe_to_out_file.md)
- [read_only_blocks_net_method_call](../../../../../functions/src/llm/tools/powershell/read_only_blocks_net_method_call.md)
- [read_only_blocks_iex_without_space](../../../../../functions/src/llm/tools/powershell/read_only_blocks_iex_without_space.md)
- [read_only_allows_gt_in_string_argument](../../../../../functions/src/llm/tools/powershell/read_only_allows_gt_in_string_argument.md)
- [read_only_blocks_redirection_with_space](../../../../../functions/src/llm/tools/powershell/read_only_blocks_redirection_with_space.md)
- [read_only_blocks_append_no_spaces](../../../../../functions/src/llm/tools/powershell/read_only_blocks_append_no_spaces.md)