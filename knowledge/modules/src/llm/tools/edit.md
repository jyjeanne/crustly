---
type: Rust Module
title: edit
resource: src/llm/tools/edit.rs#L1-L752
generated:
  by: okf-rs/0.3.0
relationships:
  imports:
  - external/super-error-validate-file-path-result-toolerror
  - external/super-file-read-cache-filefingerprint-readgate
  - external/super-r-trait-tool-toolcapability-toolexecutioncontext-toolresult
  - external/async-trait-async-trait
  - external/serde-deserialize-serialize
  - external/serde-json-value
  - external/tokio-fs
  - external/super
  - external/tempfile-tempdir
  - external/uuid-uuid
  member_of:
  - packages/crustly
---

# Contains

- [EditTool](../../../../classes/src/llm/tools/edit/EditTool.md)
- [EditOperation](../../../../classes/src/llm/tools/edit/EditOperation.md)
- [EditInput](../../../../classes/src/llm/tools/edit/EditInput.md)
- [default_true](../../../../functions/src/llm/tools/edit/default_true.md)
- [normalize_input](../../../../functions/src/llm/tools/edit/normalize_input.md)
- [name](../../../../functions/src/llm/tools/edit/EditTool/tool/name.md)
- [description](../../../../functions/src/llm/tools/edit/EditTool/tool/description.md)
- [input_schema](../../../../functions/src/llm/tools/edit/EditTool/tool/input_schema.md)
- [capabilities](../../../../functions/src/llm/tools/edit/EditTool/tool/capabilities.md)
- [requires_approval](../../../../functions/src/llm/tools/edit/EditTool/tool/requires_approval.md)
- [validate_input](../../../../functions/src/llm/tools/edit/EditTool/tool/validate_input.md)
- [execute](../../../../functions/src/llm/tools/edit/EditTool/tool/execute.md)
- [context](../../../../functions/src/llm/tools/edit/context.md)
- [seeded_context](../../../../functions/src/llm/tools/edit/seeded_context.md)
- [test_replace_with_explicit_operation_still_works](../../../../functions/src/llm/tools/edit/test_replace_with_explicit_operation_still_works.md)
- [test_qwen_code_and_claude_code_style_payload_works_with_no_operation_field](../../../../functions/src/llm/tools/edit/test_qwen_code_and_claude_code_style_payload_works_with_no_operation_field.md)
- [test_replace_rejects_non_unique_match_by_default](../../../../functions/src/llm/tools/edit/test_replace_rejects_non_unique_match_by_default.md)
- [test_replace_all_true_replaces_every_occurrence](../../../../functions/src/llm/tools/edit/test_replace_all_true_replaces_every_occurrence.md)
- [test_replace_missing_text_errors](../../../../functions/src/llm/tools/edit/test_replace_missing_text_errors.md)
- [test_line_operation_without_operation_field_is_rejected](../../../../functions/src/llm/tools/edit/test_line_operation_without_operation_field_is_rejected.md)
- [test_replace_lines_still_works](../../../../functions/src/llm/tools/edit/test_replace_lines_still_works.md)
- [test_validate_input_accepts_file_path_alias](../../../../functions/src/llm/tools/edit/test_validate_input_accepts_file_path_alias.md)
- [test_edit_rejects_a_file_never_read_this_session](../../../../functions/src/llm/tools/edit/test_edit_rejects_a_file_never_read_this_session.md)
- [test_edit_rejects_a_file_changed_since_it_was_read](../../../../functions/src/llm/tools/edit/test_edit_rejects_a_file_changed_since_it_was_read.md)
- [test_read_file_then_edit_file_succeeds](../../../../functions/src/llm/tools/edit/test_read_file_then_edit_file_succeeds.md)
- [test_consecutive_edits_do_not_require_a_re_read_between_them](../../../../functions/src/llm/tools/edit/test_consecutive_edits_do_not_require_a_re_read_between_them.md)

# Imports

- `super::error::{validate_file_path, Result, ToolError}`
- `super::file_read_cache::{FileFingerprint, ReadGate}`
- `super::r#trait::{Tool, ToolCapability, ToolExecutionContext, ToolResult}`
- `async_trait::async_trait`
- `serde::{Deserialize, Serialize}`
- `serde_json::Value`
- `tokio::fs`
- `super::*`
- `tempfile::TempDir`
- `uuid::Uuid`

# Member of

- [crustly](../../../../packages/crustly.md)