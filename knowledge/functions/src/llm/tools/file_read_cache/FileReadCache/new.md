---
type: Rust Method
title: new
resource: src/llm/tools/file_read_cache.rs#L66-L68
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/agent/service/AgentService/new
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
  - target: functions/src/llm/tools/trait/ToolExecutionContext/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn new() -> Self`

# Called by

- [new](../../../../../../functions/src/llm/agent/service/AgentService/new.md)
- [never_read_path_is_rejected](../../../../../../functions/src/llm/tools/file_read_cache/never_read_path_is_rejected.md)
- [matching_fingerprint_after_record_is_ok](../../../../../../functions/src/llm/tools/file_read_cache/matching_fingerprint_after_record_is_ok.md)
- [mismatched_fingerprint_is_stale](../../../../../../functions/src/llm/tools/file_read_cache/mismatched_fingerprint_is_stale.md)
- [distinct_paths_are_tracked_independently](../../../../../../functions/src/llm/tools/file_read_cache/distinct_paths_are_tracked_independently.md)
- [re_recording_updates_the_fingerprint](../../../../../../functions/src/llm/tools/file_read_cache/re_recording_updates_the_fingerprint.md)
- [new](../../../../../../functions/src/llm/tools/trait/ToolExecutionContext/new.md)