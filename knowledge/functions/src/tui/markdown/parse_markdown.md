---
type: Rust Function
title: parse_markdown
resource: src/tui/markdown.rs#L216-L232
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/tui/markdown/MarkdownRenderer/handle_start_tag
  - functions/src/tui/markdown/MarkdownRenderer/handle_end_tag
  - functions/src/tui/markdown/MarkdownRenderer/handle_text
  - functions/src/tui/markdown/MarkdownRenderer/handle_inline_code
  - functions/src/tui/markdown/MarkdownRenderer/flush_current_line
  - functions/src/tui/markdown/MarkdownRenderer/handle_rule
  - functions/src/tui/markdown/MarkdownRenderer/finish
  called_by:
  - functions/src/tui/markdown/markdown_escapes_backslash_before_punctuation
  - functions/src/tui/markdown/test_parse_simple_text
  - functions/src/tui/markdown/test_parse_heading
  - functions/src/tui/markdown/test_parse_code_block
  - functions/src/tui/markdown/test_parse_inline_code
  - functions/src/tui/markdown/test_parse_list
  - functions/src/tui/markdown/test_parse_horizontal_rule
  - functions/src/tui/markdown/test_empty_markdown
  - functions/src/tui/render/render_message_lines
  - functions/src/tui/render/render_streaming_response
---

# Signature

`pub fn parse_markdown(markdown: &str) -> Vec<Line<'static>>`

# Calls

- [handle_start_tag](../../../../functions/src/tui/markdown/MarkdownRenderer/handle_start_tag.md)
- [handle_end_tag](../../../../functions/src/tui/markdown/MarkdownRenderer/handle_end_tag.md)
- [handle_text](../../../../functions/src/tui/markdown/MarkdownRenderer/handle_text.md)
- [handle_inline_code](../../../../functions/src/tui/markdown/MarkdownRenderer/handle_inline_code.md)
- [flush_current_line](../../../../functions/src/tui/markdown/MarkdownRenderer/flush_current_line.md)
- [handle_rule](../../../../functions/src/tui/markdown/MarkdownRenderer/handle_rule.md)
- [finish](../../../../functions/src/tui/markdown/MarkdownRenderer/finish.md)

# Called by

- [markdown_escapes_backslash_before_punctuation](../../../../functions/src/tui/markdown/markdown_escapes_backslash_before_punctuation.md)
- [test_parse_simple_text](../../../../functions/src/tui/markdown/test_parse_simple_text.md)
- [test_parse_heading](../../../../functions/src/tui/markdown/test_parse_heading.md)
- [test_parse_code_block](../../../../functions/src/tui/markdown/test_parse_code_block.md)
- [test_parse_inline_code](../../../../functions/src/tui/markdown/test_parse_inline_code.md)
- [test_parse_list](../../../../functions/src/tui/markdown/test_parse_list.md)
- [test_parse_horizontal_rule](../../../../functions/src/tui/markdown/test_parse_horizontal_rule.md)
- [test_empty_markdown](../../../../functions/src/tui/markdown/test_empty_markdown.md)
- [render_message_lines](../../../../functions/src/tui/render/render_message_lines.md)
- [render_streaming_response](../../../../functions/src/tui/render/render_streaming_response.md)