---
type: Rust Module
title: highlight
resource: src/tui/highlight.rs#L1-L214
generated:
  by: okf-rs/0.2.0
relationships:
  imports:
  - external/once-cell-sync-lazy
  - external/ratatui-style-color-style-text-line-span
  - external/syntect-easy-highlightlines-highlighting-fontstyle-theme-themeset-parsing-syntaxreference-syntaxset-util-lineswithendings
  - external/super
  member_of:
  - packages/crustly
---

# Contains

- [get_theme](../../../functions/src/tui/highlight/get_theme.md)
- [syntect_to_ratatui_color](../../../functions/src/tui/highlight/syntect_to_ratatui_color.md)
- [syntect_style_to_ratatui](../../../functions/src/tui/highlight/syntect_style_to_ratatui.md)
- [find_syntax](../../../functions/src/tui/highlight/find_syntax.md)
- [highlight_code](../../../functions/src/tui/highlight/highlight_code.md)
- [supported_languages](../../../functions/src/tui/highlight/supported_languages.md)
- [is_language_supported](../../../functions/src/tui/highlight/is_language_supported.md)
- [test_highlight_rust](../../../functions/src/tui/highlight/test_highlight_rust.md)
- [test_highlight_python](../../../functions/src/tui/highlight/test_highlight_python.md)
- [test_highlight_javascript](../../../functions/src/tui/highlight/test_highlight_javascript.md)
- [test_highlight_unknown_language](../../../functions/src/tui/highlight/test_highlight_unknown_language.md)
- [test_supported_languages](../../../functions/src/tui/highlight/test_supported_languages.md)
- [test_is_language_supported](../../../functions/src/tui/highlight/test_is_language_supported.md)
- [test_empty_code](../../../functions/src/tui/highlight/test_empty_code.md)
- [test_code_with_special_characters](../../../functions/src/tui/highlight/test_code_with_special_characters.md)

# Imports

- `once_cell::sync::Lazy`
- `ratatui::{
    style::{Color, Style},
    text::{Line, Span},
}`
- `syntect::{
    easy::HighlightLines,
    highlighting::{FontStyle, Theme, ThemeSet},
    parsing::{SyntaxReference, SyntaxSet},
    util::LinesWithEndings,
}`
- `super::*`

# Member of

- [crustly](../../../packages/crustly.md)