---
type: Rust Module
title: plan_tool_security_tests
resource: src/llm/tools/plan_tool_security_tests.rs#L1-L223
generated:
  by: okf-rs/0.2.0
relationships:
  imports:
  - external/super-super
  - external/std-path-pathbuf
  - external/tempfile-tempdir
  - external/std-os-unix-fs-symlink
  member_of:
  - packages/crustly
---

# Contains

- [test_validate_path_within_working_directory](../../../../functions/src/llm/tools/plan_tool_security_tests/test_validate_path_within_working_directory.md)
- [test_validate_path_outside_working_directory](../../../../functions/src/llm/tools/plan_tool_security_tests/test_validate_path_outside_working_directory.md)
- [test_validate_path_traversal_attack](../../../../functions/src/llm/tools/plan_tool_security_tests/test_validate_path_traversal_attack.md)
- [test_validate_filename_pattern](../../../../functions/src/llm/tools/plan_tool_security_tests/test_validate_filename_pattern.md)
- [test_validate_filename_requires_uuid](../../../../functions/src/llm/tools/plan_tool_security_tests/test_validate_filename_requires_uuid.md)
- [test_validate_symlink_rejection](../../../../functions/src/llm/tools/plan_tool_security_tests/test_validate_symlink_rejection.md)
- [test_validate_string_empty](../../../../functions/src/llm/tools/plan_tool_security_tests/test_validate_string_empty.md)
- [test_validate_string_whitespace_only](../../../../functions/src/llm/tools/plan_tool_security_tests/test_validate_string_whitespace_only.md)
- [test_validate_string_exceeds_max_length](../../../../functions/src/llm/tools/plan_tool_security_tests/test_validate_string_exceeds_max_length.md)
- [test_validate_string_valid](../../../../functions/src/llm/tools/plan_tool_security_tests/test_validate_string_valid.md)
- [test_max_plan_file_size_constant](../../../../functions/src/llm/tools/plan_tool_security_tests/test_max_plan_file_size_constant.md)
- [test_input_validation_limits](../../../../functions/src/llm/tools/plan_tool_security_tests/test_input_validation_limits.md)
- [test_default_complexity](../../../../functions/src/llm/tools/plan_tool_security_tests/test_default_complexity.md)
- [test_validate_title_at_limit](../../../../functions/src/llm/tools/plan_tool_security_tests/test_validate_title_at_limit.md)
- [test_validate_title_one_over_limit](../../../../functions/src/llm/tools/plan_tool_security_tests/test_validate_title_one_over_limit.md)
- [test_validate_description_at_limit](../../../../functions/src/llm/tools/plan_tool_security_tests/test_validate_description_at_limit.md)
- [test_validate_context_at_limit](../../../../functions/src/llm/tools/plan_tool_security_tests/test_validate_context_at_limit.md)
- [test_filename_with_special_characters](../../../../functions/src/llm/tools/plan_tool_security_tests/test_filename_with_special_characters.md)
- [test_filename_with_null_byte](../../../../functions/src/llm/tools/plan_tool_security_tests/test_filename_with_null_byte.md)
- [test_validate_plan_file_path_canonical](../../../../functions/src/llm/tools/plan_tool_security_tests/test_validate_plan_file_path_canonical.md)

# Imports

- `super::super::*`
- `std::path::PathBuf`
- `tempfile::TempDir`
- `std::os::unix::fs::symlink`

# Member of

- [crustly](../../../../packages/crustly.md)