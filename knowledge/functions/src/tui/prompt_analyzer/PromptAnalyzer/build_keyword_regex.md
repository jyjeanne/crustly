---
type: Rust Method
title: build_keyword_regex
resource: src/tui/prompt_analyzer.rs#L117-L124
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/tui/prompt_analyzer/PromptAnalyzer/new
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn build_keyword_regex(keywords: &[&str]) -> Regex`

# Called by

- [new](../../../../../functions/src/tui/prompt_analyzer/PromptAnalyzer/new.md)