---
type: Rust Module
title: apply_patch
resource: src/llm/tools/apply_patch.rs#L1-L907
generated:
  by: okf-rs/0.3.0
relationships:
  imports:
  - external/super-error-validate-file-path-validate-path-safety-result-toolerror
  - external/super-file-read-cache-filefingerprint-readgate
  - external/super-r-trait-tool-toolcapability-toolexecutioncontext-toolresult
  - external/async-trait-async-trait
  - external/serde-deserialize
  - external/serde-json-value
  - external/std-path-pathbuf
  - external/tokio-fs
  - external/super
  - external/tempfile-tempdir
  - external/uuid-uuid
  member_of:
  - packages/crustly
---

# Contains

- [ApplyPatchTool](../../../../classes/src/llm/tools/apply_patch/ApplyPatchTool.md)
- [ApplyPatchInput](../../../../classes/src/llm/tools/apply_patch/ApplyPatchInput.md)
- [HunkLine](../../../../classes/src/llm/tools/apply_patch/HunkLine.md)
- [Hunk](../../../../classes/src/llm/tools/apply_patch/Hunk.md)
- [FileOp](../../../../classes/src/llm/tools/apply_patch/FileOp.md)
- [parse_patch](../../../../functions/src/llm/tools/apply_patch/parse_patch.md)
- [find_subsequence](../../../../functions/src/llm/tools/apply_patch/find_subsequence.md)
- [apply_hunks](../../../../functions/src/llm/tools/apply_patch/apply_hunks.md)
- [PlannedAction](../../../../classes/src/llm/tools/apply_patch/PlannedAction.md)
- [name](../../../../functions/src/llm/tools/apply_patch/ApplyPatchTool/tool/name.md)
- [description](../../../../functions/src/llm/tools/apply_patch/ApplyPatchTool/tool/description.md)
- [input_schema](../../../../functions/src/llm/tools/apply_patch/ApplyPatchTool/tool/input_schema.md)
- [capabilities](../../../../functions/src/llm/tools/apply_patch/ApplyPatchTool/tool/capabilities.md)
- [requires_approval](../../../../functions/src/llm/tools/apply_patch/ApplyPatchTool/tool/requires_approval.md)
- [validate_input](../../../../functions/src/llm/tools/apply_patch/ApplyPatchTool/tool/validate_input.md)
- [execute](../../../../functions/src/llm/tools/apply_patch/ApplyPatchTool/tool/execute.md)
- [context](../../../../functions/src/llm/tools/apply_patch/context.md)
- [seeded_context](../../../../functions/src/llm/tools/apply_patch/seeded_context.md)
- [parse_rejects_missing_begin_marker](../../../../functions/src/llm/tools/apply_patch/parse_rejects_missing_begin_marker.md)
- [parse_rejects_missing_end_marker](../../../../functions/src/llm/tools/apply_patch/parse_rejects_missing_end_marker.md)
- [parse_add_file_collects_plus_prefixed_lines](../../../../functions/src/llm/tools/apply_patch/parse_add_file_collects_plus_prefixed_lines.md)
- [parse_multiple_file_ops_in_one_patch](../../../../functions/src/llm/tools/apply_patch/parse_multiple_file_ops_in_one_patch.md)
- [parse_update_with_move_to](../../../../functions/src/llm/tools/apply_patch/parse_update_with_move_to.md)
- [apply_hunks_replaces_matched_context](../../../../functions/src/llm/tools/apply_patch/apply_hunks_replaces_matched_context.md)
- [apply_hunks_second_hunk_searches_after_first](../../../../functions/src/llm/tools/apply_patch/apply_hunks_second_hunk_searches_after_first.md)
- [apply_hunks_errors_when_context_not_found](../../../../functions/src/llm/tools/apply_patch/apply_hunks_errors_when_context_not_found.md)
- [execute_updates_an_existing_file](../../../../functions/src/llm/tools/apply_patch/execute_updates_an_existing_file.md)
- [execute_adds_a_new_file](../../../../functions/src/llm/tools/apply_patch/execute_adds_a_new_file.md)
- [execute_add_file_that_already_exists_fails](../../../../functions/src/llm/tools/apply_patch/execute_add_file_that_already_exists_fails.md)
- [execute_deletes_a_file](../../../../functions/src/llm/tools/apply_patch/execute_deletes_a_file.md)
- [execute_renames_via_move_to](../../../../functions/src/llm/tools/apply_patch/execute_renames_via_move_to.md)
- [execute_applies_multiple_file_ops_in_one_patch](../../../../functions/src/llm/tools/apply_patch/execute_applies_multiple_file_ops_in_one_patch.md)
- [execute_is_atomic_across_files_on_failure](../../../../functions/src/llm/tools/apply_patch/execute_is_atomic_across_files_on_failure.md)
- [execute_blocked_in_read_only_mode](../../../../functions/src/llm/tools/apply_patch/execute_blocked_in_read_only_mode.md)
- [validate_input_rejects_malformed_patch](../../../../functions/src/llm/tools/apply_patch/validate_input_rejects_malformed_patch.md)
- [execute_update_rejects_a_file_never_read_this_session](../../../../functions/src/llm/tools/apply_patch/execute_update_rejects_a_file_never_read_this_session.md)
- [execute_add_and_delete_need_no_prior_read](../../../../functions/src/llm/tools/apply_patch/execute_add_and_delete_need_no_prior_read.md)

# Imports

- `super::error::{validate_file_path, validate_path_safety, Result, ToolError}`
- `super::file_read_cache::{FileFingerprint, ReadGate}`
- `super::r#trait::{Tool, ToolCapability, ToolExecutionContext, ToolResult}`
- `async_trait::async_trait`
- `serde::Deserialize`
- `serde_json::Value`
- `std::path::PathBuf`
- `tokio::fs`
- `super::*`
- `tempfile::TempDir`
- `uuid::Uuid`

# Member of

- [crustly](../../../../packages/crustly.md)