---
type: Rust Method
title: record
resource: src/llm/tools/file_read_cache.rs#L76-L81
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/llm/tools/apply_patch/ApplyPatchTool/tool/execute
  - functions/src/llm/tools/apply_patch/seeded_context
  - functions/src/llm/tools/apply_patch/execute_is_atomic_across_files_on_failure
  - functions/src/llm/tools/edit/EditTool/tool/execute
  - functions/src/llm/tools/edit/seeded_context
  - functions/src/llm/tools/file_read_cache/matching_fingerprint_after_record_is_ok
  - functions/src/llm/tools/file_read_cache/mismatched_fingerprint_is_stale
  - functions/src/llm/tools/file_read_cache/distinct_paths_are_tracked_independently
  - functions/src/llm/tools/file_read_cache/re_recording_updates_the_fingerprint
  - functions/src/llm/tools/read/ReadTool/tool/execute
  - functions/src/llm/tools/write/WriteTool/tool/execute
  - functions/src/llm/tools/write/test_overwrite_existing_file
  - functions/src/llm/tools/write/test_overwrite_rejects_a_file_changed_since_it_was_read
---

# Signature

`pub fn record(&self, path: &Path, fingerprint: FileFingerprint)`

# Called by

- [execute](../../../../../../functions/src/llm/tools/apply_patch/ApplyPatchTool/tool/execute.md)
- [seeded_context](../../../../../../functions/src/llm/tools/apply_patch/seeded_context.md)
- [execute_is_atomic_across_files_on_failure](../../../../../../functions/src/llm/tools/apply_patch/execute_is_atomic_across_files_on_failure.md)
- [execute](../../../../../../functions/src/llm/tools/edit/EditTool/tool/execute.md)
- [seeded_context](../../../../../../functions/src/llm/tools/edit/seeded_context.md)
- [matching_fingerprint_after_record_is_ok](../../../../../../functions/src/llm/tools/file_read_cache/matching_fingerprint_after_record_is_ok.md)
- [mismatched_fingerprint_is_stale](../../../../../../functions/src/llm/tools/file_read_cache/mismatched_fingerprint_is_stale.md)
- [distinct_paths_are_tracked_independently](../../../../../../functions/src/llm/tools/file_read_cache/distinct_paths_are_tracked_independently.md)
- [re_recording_updates_the_fingerprint](../../../../../../functions/src/llm/tools/file_read_cache/re_recording_updates_the_fingerprint.md)
- [execute](../../../../../../functions/src/llm/tools/read/ReadTool/tool/execute.md)
- [execute](../../../../../../functions/src/llm/tools/write/WriteTool/tool/execute.md)
- [test_overwrite_existing_file](../../../../../../functions/src/llm/tools/write/test_overwrite_existing_file.md)
- [test_overwrite_rejects_a_file_changed_since_it_was_read](../../../../../../functions/src/llm/tools/write/test_overwrite_rejects_a_file_changed_since_it_was_read.md)