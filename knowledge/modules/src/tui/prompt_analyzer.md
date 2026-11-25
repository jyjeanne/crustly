---
type: Rust Module
title: prompt_analyzer
resource: src/tui/prompt_analyzer.rs#L1-L392
generated:
  by: okf-rs/0.8.0
relationships:
  imports:
  - target: external/regex-regex
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-llm-provider-router-modeltier
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [PromptAnalyzer](../../../classes/src/tui/prompt_analyzer/PromptAnalyzer.md)
- [new](../../../functions/src/tui/prompt_analyzer/PromptAnalyzer/new.md)
- [build_keyword_regex](../../../functions/src/tui/prompt_analyzer/PromptAnalyzer/build_keyword_regex.md)
- [analyze_and_transform](../../../functions/src/tui/prompt_analyzer/PromptAnalyzer/analyze_and_transform.md)
- [analyze_and_transform_for](../../../functions/src/tui/prompt_analyzer/PromptAnalyzer/analyze_and_transform_for.md)
- [default](../../../functions/src/tui/prompt_analyzer/PromptAnalyzer/default/default.md)
- [classify_tier](../../../functions/src/tui/prompt_analyzer/PromptAnalyzer/classify_tier.md)
- [test_plan_detection](../../../functions/src/tui/prompt_analyzer/test_plan_detection.md)
- [hints_skip_tools_that_are_not_available](../../../functions/src/tui/prompt_analyzer/hints_skip_tools_that_are_not_available.md)
- [test_read_file_detection](../../../functions/src/tui/prompt_analyzer/test_read_file_detection.md)
- [test_search_detection](../../../functions/src/tui/prompt_analyzer/test_search_detection.md)
- [test_multiple_detections](../../../functions/src/tui/prompt_analyzer/test_multiple_detections.md)
- [test_no_detection](../../../functions/src/tui/prompt_analyzer/test_no_detection.md)
- [test_case_insensitive](../../../functions/src/tui/prompt_analyzer/test_case_insensitive.md)
- [test_web_search_detection](../../../functions/src/tui/prompt_analyzer/test_web_search_detection.md)
- [test_bash_detection](../../../functions/src/tui/prompt_analyzer/test_bash_detection.md)

# Imports

- `regex::Regex`
- `crate::llm::provider::router::ModelTier`
- `super::*`

# Member of

- [crustly](../../../packages/crustly.md)