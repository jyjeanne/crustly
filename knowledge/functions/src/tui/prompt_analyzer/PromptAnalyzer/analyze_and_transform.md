---
type: Rust Method
title: analyze_and_transform
resource: src/tui/prompt_analyzer.rs#L127-L197
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/is_empty
  called_by:
  - functions/src/tui/app/App/send_message
  - functions/src/tui/prompt_analyzer/test_plan_detection
  - functions/src/tui/prompt_analyzer/test_read_file_detection
  - functions/src/tui/prompt_analyzer/test_search_detection
  - functions/src/tui/prompt_analyzer/test_multiple_detections
  - functions/src/tui/prompt_analyzer/test_no_detection
  - functions/src/tui/prompt_analyzer/test_case_insensitive
  - functions/src/tui/prompt_analyzer/test_web_search_detection
  - functions/src/tui/prompt_analyzer/test_bash_detection
---

# Signature

`pub fn analyze_and_transform(&self, prompt: &str) -> String`

# Calls

- [is_empty](../../../../../functions/src/config/secrets/SecretString/is_empty.md)

# Called by

- [send_message](../../../../../functions/src/tui/app/App/send_message.md)
- [test_plan_detection](../../../../../functions/src/tui/prompt_analyzer/test_plan_detection.md)
- [test_read_file_detection](../../../../../functions/src/tui/prompt_analyzer/test_read_file_detection.md)
- [test_search_detection](../../../../../functions/src/tui/prompt_analyzer/test_search_detection.md)
- [test_multiple_detections](../../../../../functions/src/tui/prompt_analyzer/test_multiple_detections.md)
- [test_no_detection](../../../../../functions/src/tui/prompt_analyzer/test_no_detection.md)
- [test_case_insensitive](../../../../../functions/src/tui/prompt_analyzer/test_case_insensitive.md)
- [test_web_search_detection](../../../../../functions/src/tui/prompt_analyzer/test_web_search_detection.md)
- [test_bash_detection](../../../../../functions/src/tui/prompt_analyzer/test_bash_detection.md)