---
type: Rust Function
title: test_bash_detection
resource: src/tui/prompt_analyzer.rs#L384-L391
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/prompt_analyzer/PromptAnalyzer/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/prompt_analyzer/PromptAnalyzer/analyze_and_transform
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_bash_detection()`

# Calls

- [new](../../../../functions/src/tui/prompt_analyzer/PromptAnalyzer/new.md)
- [analyze_and_transform](../../../../functions/src/tui/prompt_analyzer/PromptAnalyzer/analyze_and_transform.md)