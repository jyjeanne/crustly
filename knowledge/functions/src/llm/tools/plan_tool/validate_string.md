---
type: Rust Function
title: validate_string
resource: src/llm/tools/plan_tool.rs#L175-L193
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/tools/plan_tool/PlanTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/plan_tool_security_tests/test_validate_string_empty
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/plan_tool_security_tests/test_validate_string_whitespace_only
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/plan_tool_security_tests/test_validate_string_exceeds_max_length
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/plan_tool_security_tests/test_validate_string_valid
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/plan_tool_security_tests/test_validate_title_at_limit
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/plan_tool_security_tests/test_validate_title_one_over_limit
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/plan_tool_security_tests/test_validate_description_at_limit
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/plan_tool_security_tests/test_validate_context_at_limit
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn validate_string(s: &str, max_len: usize, field_name: &str) -> Result<()>`

# Called by

- [execute](../../../../../functions/src/llm/tools/plan_tool/PlanTool/tool/execute.md)
- [test_validate_string_empty](../../../../../functions/src/llm/tools/plan_tool_security_tests/test_validate_string_empty.md)
- [test_validate_string_whitespace_only](../../../../../functions/src/llm/tools/plan_tool_security_tests/test_validate_string_whitespace_only.md)
- [test_validate_string_exceeds_max_length](../../../../../functions/src/llm/tools/plan_tool_security_tests/test_validate_string_exceeds_max_length.md)
- [test_validate_string_valid](../../../../../functions/src/llm/tools/plan_tool_security_tests/test_validate_string_valid.md)
- [test_validate_title_at_limit](../../../../../functions/src/llm/tools/plan_tool_security_tests/test_validate_title_at_limit.md)
- [test_validate_title_one_over_limit](../../../../../functions/src/llm/tools/plan_tool_security_tests/test_validate_title_one_over_limit.md)
- [test_validate_description_at_limit](../../../../../functions/src/llm/tools/plan_tool_security_tests/test_validate_description_at_limit.md)
- [test_validate_context_at_limit](../../../../../functions/src/llm/tools/plan_tool_security_tests/test_validate_context_at_limit.md)