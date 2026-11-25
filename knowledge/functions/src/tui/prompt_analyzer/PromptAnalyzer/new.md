---
type: Rust Method
title: new
resource: src/tui/prompt_analyzer.rs#L104-L114
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/prompt_analyzer/PromptAnalyzer/build_keyword_regex
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/app/App/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/prompt_analyzer/PromptAnalyzer/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/prompt_analyzer/test_plan_detection
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/prompt_analyzer/hints_skip_tools_that_are_not_available
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/prompt_analyzer/test_read_file_detection
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/prompt_analyzer/test_search_detection
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/prompt_analyzer/test_multiple_detections
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/prompt_analyzer/test_no_detection
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/prompt_analyzer/test_case_insensitive
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/prompt_analyzer/test_web_search_detection
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/prompt_analyzer/test_bash_detection
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/model_routing_test/complex_prompt_routes_to_powerful_tier
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/model_routing_test/simple_prompt_routes_to_fast_tier
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/model_routing_test/neutral_prompt_routes_to_balanced_tier
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn new() -> Self`

# Calls

- [build_keyword_regex](../../../../../functions/src/tui/prompt_analyzer/PromptAnalyzer/build_keyword_regex.md)

# Called by

- [send_message_with_tools_inner](../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)
- [new](../../../../../functions/src/tui/app/App/new.md)
- [default](../../../../../functions/src/tui/prompt_analyzer/PromptAnalyzer/default/default.md)
- [test_plan_detection](../../../../../functions/src/tui/prompt_analyzer/test_plan_detection.md)
- [hints_skip_tools_that_are_not_available](../../../../../functions/src/tui/prompt_analyzer/hints_skip_tools_that_are_not_available.md)
- [test_read_file_detection](../../../../../functions/src/tui/prompt_analyzer/test_read_file_detection.md)
- [test_search_detection](../../../../../functions/src/tui/prompt_analyzer/test_search_detection.md)
- [test_multiple_detections](../../../../../functions/src/tui/prompt_analyzer/test_multiple_detections.md)
- [test_no_detection](../../../../../functions/src/tui/prompt_analyzer/test_no_detection.md)
- [test_case_insensitive](../../../../../functions/src/tui/prompt_analyzer/test_case_insensitive.md)
- [test_web_search_detection](../../../../../functions/src/tui/prompt_analyzer/test_web_search_detection.md)
- [test_bash_detection](../../../../../functions/src/tui/prompt_analyzer/test_bash_detection.md)
- [complex_prompt_routes_to_powerful_tier](../../../../../functions/tests/model_routing_test/complex_prompt_routes_to_powerful_tier.md)
- [simple_prompt_routes_to_fast_tier](../../../../../functions/tests/model_routing_test/simple_prompt_routes_to_fast_tier.md)
- [neutral_prompt_routes_to_balanced_tier](../../../../../functions/tests/model_routing_test/neutral_prompt_routes_to_balanced_tier.md)