---
type: Rust Method
title: check
resource: src/llm/tools/file_read_cache.rs#L85-L92
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/tools/apply_patch/ApplyPatchTool/tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/edit/EditTool/tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/file_read_cache/never_read_path_is_rejected
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/file_read_cache/matching_fingerprint_after_record_is_ok
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/file_read_cache/mismatched_fingerprint_is_stale
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/file_read_cache/distinct_paths_are_tracked_independently
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/file_read_cache/re_recording_updates_the_fingerprint
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/write/WriteTool/tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn check(&self, path: &Path, current: FileFingerprint) -> ReadGate`

# Called by

- [execute](../../../../../../functions/src/llm/tools/apply_patch/ApplyPatchTool/tool/execute.md)
- [execute](../../../../../../functions/src/llm/tools/edit/EditTool/tool/execute.md)
- [never_read_path_is_rejected](../../../../../../functions/src/llm/tools/file_read_cache/never_read_path_is_rejected.md)
- [matching_fingerprint_after_record_is_ok](../../../../../../functions/src/llm/tools/file_read_cache/matching_fingerprint_after_record_is_ok.md)
- [mismatched_fingerprint_is_stale](../../../../../../functions/src/llm/tools/file_read_cache/mismatched_fingerprint_is_stale.md)
- [distinct_paths_are_tracked_independently](../../../../../../functions/src/llm/tools/file_read_cache/distinct_paths_are_tracked_independently.md)
- [re_recording_updates_the_fingerprint](../../../../../../functions/src/llm/tools/file_read_cache/re_recording_updates_the_fingerprint.md)
- [execute](../../../../../../functions/src/llm/tools/write/WriteTool/tool/execute.md)