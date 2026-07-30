---
type: Rust Module
title: error
resource: src/tui/error.rs#L1-L297
generated:
  by: okf-rs/0.2.0
relationships:
  imports:
  - external/chrono-datetime-utc
  - external/ratatui-style-color
  - external/super
  member_of:
  - packages/crustly
---

# Contains

- [ErrorSeverity](../../../classes/src/tui/error/ErrorSeverity.md)
- [color](../../../functions/src/tui/error/ErrorSeverity/color.md)
- [prefix](../../../functions/src/tui/error/ErrorSeverity/prefix.md)
- [name](../../../functions/src/tui/error/ErrorSeverity/name.md)
- [ErrorCategory](../../../classes/src/tui/error/ErrorCategory.md)
- [name](../../../functions/src/tui/error/ErrorCategory/name.md)
- [ErrorInfo](../../../classes/src/tui/error/ErrorInfo.md)
- [new](../../../functions/src/tui/error/ErrorInfo/new.md)
- [info](../../../functions/src/tui/error/ErrorInfo/info.md)
- [warning](../../../functions/src/tui/error/ErrorInfo/warning.md)
- [error](../../../functions/src/tui/error/ErrorInfo/error.md)
- [critical](../../../functions/src/tui/error/ErrorInfo/critical.md)
- [with_context](../../../functions/src/tui/error/ErrorInfo/with_context.md)
- [with_retry](../../../functions/src/tui/error/ErrorInfo/with_retry.md)
- [summary](../../../functions/src/tui/error/ErrorInfo/summary.md)
- [description](../../../functions/src/tui/error/ErrorInfo/description.md)
- [from](../../../functions/src/tui/error/ErrorInfo/from-string/from.md)
- [from](../../../functions/src/tui/error/ErrorInfo/from-str/from.md)
- [test_error_severity_color](../../../functions/src/tui/error/test_error_severity_color.md)
- [test_error_info_creation](../../../functions/src/tui/error/test_error_info_creation.md)
- [test_error_info_with_retry](../../../functions/src/tui/error/test_error_info_with_retry.md)
- [test_error_info_summary](../../../functions/src/tui/error/test_error_info_summary.md)
- [test_error_info_from_string](../../../functions/src/tui/error/test_error_info_from_string.md)

# Imports

- `chrono::{DateTime, Utc}`
- `ratatui::style::Color`
- `super::*`

# Member of

- [crustly](../../../packages/crustly.md)