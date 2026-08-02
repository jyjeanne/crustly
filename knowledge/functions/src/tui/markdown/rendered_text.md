---
type: Rust Function
title: rendered_text
resource: src/tui/markdown.rs#L270-L281
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/tui/markdown/plain_text_keeps_windows_path_backslashes
  - functions/src/tui/markdown/plain_text_keeps_markdown_syntax_literal
  - functions/src/tui/markdown/plain_text_preserves_line_structure
  - functions/src/tui/markdown/markdown_escapes_backslash_before_punctuation
---

# Signature

`fn rendered_text(lines: &[Line<'static>]) -> String`

# Called by

- [plain_text_keeps_windows_path_backslashes](../../../../functions/src/tui/markdown/plain_text_keeps_windows_path_backslashes.md)
- [plain_text_keeps_markdown_syntax_literal](../../../../functions/src/tui/markdown/plain_text_keeps_markdown_syntax_literal.md)
- [plain_text_preserves_line_structure](../../../../functions/src/tui/markdown/plain_text_preserves_line_structure.md)
- [markdown_escapes_backslash_before_punctuation](../../../../functions/src/tui/markdown/markdown_escapes_backslash_before_punctuation.md)