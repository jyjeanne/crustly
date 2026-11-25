---
type: Rust Method
title: analyze_and_transform
resource: src/tui/prompt_analyzer.rs#L128-L130
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/prompt_analyzer/PromptAnalyzer/analyze_and_transform_for
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/prompt_analyzer/test_plan_detection
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/prompt_analyzer/test_read_file_detection
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/prompt_analyzer/test_search_detection
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/prompt_analyzer/test_multiple_detections
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/prompt_analyzer/test_no_detection
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/prompt_analyzer/test_case_insensitive
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/prompt_analyzer/test_web_search_detection
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/prompt_analyzer/test_bash_detection
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn analyze_and_transform(&self, prompt: &str) -> String`

# Calls

- [analyze_and_transform_for](../../../../../functions/src/tui/prompt_analyzer/PromptAnalyzer/analyze_and_transform_for.md)

# Called by

- [test_plan_detection](../../../../../functions/src/tui/prompt_analyzer/test_plan_detection.md)
- [test_read_file_detection](../../../../../functions/src/tui/prompt_analyzer/test_read_file_detection.md)
- [test_search_detection](../../../../../functions/src/tui/prompt_analyzer/test_search_detection.md)
- [test_multiple_detections](../../../../../functions/src/tui/prompt_analyzer/test_multiple_detections.md)
- [test_no_detection](../../../../../functions/src/tui/prompt_analyzer/test_no_detection.md)
- [test_case_insensitive](../../../../../functions/src/tui/prompt_analyzer/test_case_insensitive.md)
- [test_web_search_detection](../../../../../functions/src/tui/prompt_analyzer/test_web_search_detection.md)
- [test_bash_detection](../../../../../functions/src/tui/prompt_analyzer/test_bash_detection.md)