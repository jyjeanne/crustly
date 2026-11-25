---
type: Rust Method
title: analyze_and_transform_for
resource: src/tui/prompt_analyzer.rs#L136-L210
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/tui/app/App/send_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/prompt_analyzer/PromptAnalyzer/analyze_and_transform
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/prompt_analyzer/hints_skip_tools_that_are_not_available
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn analyze_and_transform_for( &self, prompt: &str, available: impl Fn(&str) -> bool, ) -> String`

# Called by

- [send_message](../../../../../functions/src/tui/app/App/send_message.md)
- [analyze_and_transform](../../../../../functions/src/tui/prompt_analyzer/PromptAnalyzer/analyze_and_transform.md)
- [hints_skip_tools_that_are_not_available](../../../../../functions/src/tui/prompt_analyzer/hints_skip_tools_that_are_not_available.md)