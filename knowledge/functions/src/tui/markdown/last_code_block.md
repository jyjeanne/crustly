---
type: Rust Function
title: last_code_block
resource: src/tui/markdown.rs#L238-L262
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/tui/app/App/copy_last_response_to_clipboard
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/markdown/last_code_block_extracts_fenced_content
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/markdown/last_code_block_returns_the_last_of_multiple_blocks
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/markdown/last_code_block_returns_none_without_any_code
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn last_code_block(markdown: &str) -> Option<String>`

# Called by

- [copy_last_response_to_clipboard](../../../../functions/src/tui/app/App/copy_last_response_to_clipboard.md)
- [last_code_block_extracts_fenced_content](../../../../functions/src/tui/markdown/last_code_block_extracts_fenced_content.md)
- [last_code_block_returns_the_last_of_multiple_blocks](../../../../functions/src/tui/markdown/last_code_block_returns_the_last_of_multiple_blocks.md)
- [last_code_block_returns_none_without_any_code](../../../../functions/src/tui/markdown/last_code_block_returns_none_without_any_code.md)