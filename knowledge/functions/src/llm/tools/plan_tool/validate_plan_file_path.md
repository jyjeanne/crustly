---
type: Rust Function
title: validate_plan_file_path
resource: src/llm/tools/plan_tool.rs#L127-L164
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/len
  called_by:
  - functions/src/llm/tools/plan_tool/PlanTool/tool/execute
  - functions/src/llm/tools/plan_tool_security_tests/test_validate_path_within_working_directory
  - functions/src/llm/tools/plan_tool_security_tests/test_validate_path_outside_working_directory
  - functions/src/llm/tools/plan_tool_security_tests/test_validate_path_traversal_attack
  - functions/src/llm/tools/plan_tool_security_tests/test_validate_filename_pattern
  - functions/src/llm/tools/plan_tool_security_tests/test_validate_filename_requires_uuid
  - functions/src/llm/tools/plan_tool_security_tests/test_validate_symlink_rejection
  - functions/src/llm/tools/plan_tool_security_tests/test_filename_with_special_characters
  - functions/src/llm/tools/plan_tool_security_tests/test_filename_with_null_byte
  - functions/src/llm/tools/plan_tool_security_tests/test_validate_plan_file_path_canonical
---

# Signature

`fn validate_plan_file_path(path: &Path, working_dir: &Path) -> Result<()>`

# Calls

- [len](../../../../../functions/src/config/secrets/SecretString/len.md)

# Called by

- [execute](../../../../../functions/src/llm/tools/plan_tool/PlanTool/tool/execute.md)
- [test_validate_path_within_working_directory](../../../../../functions/src/llm/tools/plan_tool_security_tests/test_validate_path_within_working_directory.md)
- [test_validate_path_outside_working_directory](../../../../../functions/src/llm/tools/plan_tool_security_tests/test_validate_path_outside_working_directory.md)
- [test_validate_path_traversal_attack](../../../../../functions/src/llm/tools/plan_tool_security_tests/test_validate_path_traversal_attack.md)
- [test_validate_filename_pattern](../../../../../functions/src/llm/tools/plan_tool_security_tests/test_validate_filename_pattern.md)
- [test_validate_filename_requires_uuid](../../../../../functions/src/llm/tools/plan_tool_security_tests/test_validate_filename_requires_uuid.md)
- [test_validate_symlink_rejection](../../../../../functions/src/llm/tools/plan_tool_security_tests/test_validate_symlink_rejection.md)
- [test_filename_with_special_characters](../../../../../functions/src/llm/tools/plan_tool_security_tests/test_filename_with_special_characters.md)
- [test_filename_with_null_byte](../../../../../functions/src/llm/tools/plan_tool_security_tests/test_filename_with_null_byte.md)
- [test_validate_plan_file_path_canonical](../../../../../functions/src/llm/tools/plan_tool_security_tests/test_validate_plan_file_path_canonical.md)