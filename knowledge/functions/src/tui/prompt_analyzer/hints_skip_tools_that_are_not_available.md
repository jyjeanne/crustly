---
type: Rust Function
title: hints_skip_tools_that_are_not_available
resource: src/tui/prompt_analyzer.rs#L311-L322
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
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/prompt_analyzer/PromptAnalyzer/analyze_and_transform_for
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn hints_skip_tools_that_are_not_available()`

# Calls

- [new](../../../../functions/src/tui/prompt_analyzer/PromptAnalyzer/new.md)
- [contains](../../../../functions/src/memory/vector_index/VectorIndex/contains.md)
- [analyze_and_transform_for](../../../../functions/src/tui/prompt_analyzer/PromptAnalyzer/analyze_and_transform_for.md)