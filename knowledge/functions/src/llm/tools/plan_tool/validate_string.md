---
type: Rust Function
title: validate_string
resource: src/llm/tools/plan_tool.rs#L175-L193
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/is_empty
  - functions/src/config/secrets/SecretString/len
  called_by:
  - functions/src/llm/tools/plan_tool/PlanTool/tool/execute
  - functions/src/llm/tools/plan_tool_security_tests/test_validate_string_empty
  - functions/src/llm/tools/plan_tool_security_tests/test_validate_string_whitespace_only
  - functions/src/llm/tools/plan_tool_security_tests/test_validate_string_exceeds_max_length
  - functions/src/llm/tools/plan_tool_security_tests/test_validate_string_valid
  - functions/src/llm/tools/plan_tool_security_tests/test_validate_title_at_limit
  - functions/src/llm/tools/plan_tool_security_tests/test_validate_title_one_over_limit
  - functions/src/llm/tools/plan_tool_security_tests/test_validate_description_at_limit
  - functions/src/llm/tools/plan_tool_security_tests/test_validate_context_at_limit
---

# Signature

`fn validate_string(s: &str, max_len: usize, field_name: &str) -> Result<()>`

# Calls

- [is_empty](../../../../../functions/src/config/secrets/SecretString/is_empty.md)
- [len](../../../../../functions/src/config/secrets/SecretString/len.md)

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