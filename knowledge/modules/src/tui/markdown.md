---
type: Rust Module
title: markdown
resource: src/tui/markdown.rs#L1-L401
generated:
  by: okf-rs/0.2.0
relationships:
  imports:
  - external/pulldown-cmark-codeblockkind-event-parser-tag-tagend
  - external/ratatui-style-color-modifier-style-text-line-span
  - external/super-highlight-highlight-code
  - external/super
  member_of:
  - packages/crustly
---

# Contains

- [parse_plain_text](../../../functions/src/tui/markdown/parse_plain_text.md)
- [MarkdownRenderer](../../../classes/src/tui/markdown/MarkdownRenderer.md)
- [new](../../../functions/src/tui/markdown/MarkdownRenderer/new.md)
- [flush_current_line](../../../functions/src/tui/markdown/MarkdownRenderer/flush_current_line.md)
- [start_code_block](../../../functions/src/tui/markdown/MarkdownRenderer/start_code_block.md)
- [handle_start_tag](../../../functions/src/tui/markdown/MarkdownRenderer/handle_start_tag.md)
- [end_heading](../../../functions/src/tui/markdown/MarkdownRenderer/end_heading.md)
- [end_code_block](../../../functions/src/tui/markdown/MarkdownRenderer/end_code_block.md)
- [end_list](../../../functions/src/tui/markdown/MarkdownRenderer/end_list.md)
- [end_paragraph](../../../functions/src/tui/markdown/MarkdownRenderer/end_paragraph.md)
- [handle_end_tag](../../../functions/src/tui/markdown/MarkdownRenderer/handle_end_tag.md)
- [handle_text](../../../functions/src/tui/markdown/MarkdownRenderer/handle_text.md)
- [handle_inline_code](../../../functions/src/tui/markdown/MarkdownRenderer/handle_inline_code.md)
- [handle_rule](../../../functions/src/tui/markdown/MarkdownRenderer/handle_rule.md)
- [finish](../../../functions/src/tui/markdown/MarkdownRenderer/finish.md)
- [parse_markdown](../../../functions/src/tui/markdown/parse_markdown.md)
- [last_code_block](../../../functions/src/tui/markdown/last_code_block.md)
- [rendered_text](../../../functions/src/tui/markdown/rendered_text.md)
- [plain_text_keeps_windows_path_backslashes](../../../functions/src/tui/markdown/plain_text_keeps_windows_path_backslashes.md)
- [plain_text_keeps_markdown_syntax_literal](../../../functions/src/tui/markdown/plain_text_keeps_markdown_syntax_literal.md)
- [plain_text_preserves_line_structure](../../../functions/src/tui/markdown/plain_text_preserves_line_structure.md)
- [markdown_escapes_backslash_before_punctuation](../../../functions/src/tui/markdown/markdown_escapes_backslash_before_punctuation.md)
- [test_parse_simple_text](../../../functions/src/tui/markdown/test_parse_simple_text.md)
- [test_parse_heading](../../../functions/src/tui/markdown/test_parse_heading.md)
- [test_parse_code_block](../../../functions/src/tui/markdown/test_parse_code_block.md)
- [last_code_block_extracts_fenced_content](../../../functions/src/tui/markdown/last_code_block_extracts_fenced_content.md)
- [last_code_block_returns_the_last_of_multiple_blocks](../../../functions/src/tui/markdown/last_code_block_returns_the_last_of_multiple_blocks.md)
- [last_code_block_returns_none_without_any_code](../../../functions/src/tui/markdown/last_code_block_returns_none_without_any_code.md)
- [test_parse_inline_code](../../../functions/src/tui/markdown/test_parse_inline_code.md)
- [test_parse_list](../../../functions/src/tui/markdown/test_parse_list.md)
- [test_parse_horizontal_rule](../../../functions/src/tui/markdown/test_parse_horizontal_rule.md)
- [test_empty_markdown](../../../functions/src/tui/markdown/test_empty_markdown.md)

# Imports

- `pulldown_cmark::{CodeBlockKind, Event, Parser, Tag, TagEnd}`
- `ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
}`
- `super::highlight::highlight_code`
- `super::*`

# Member of

- [crustly](../../../packages/crustly.md)