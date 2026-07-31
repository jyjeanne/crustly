---
type: Rust Function
title: fp
resource: src/llm/tools/file_read_cache.rs#L99-L104
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/llm/tools/file_read_cache/matching_fingerprint_after_record_is_ok
  - functions/src/llm/tools/file_read_cache/mismatched_fingerprint_is_stale
  - functions/src/llm/tools/file_read_cache/distinct_paths_are_tracked_independently
  - functions/src/llm/tools/file_read_cache/re_recording_updates_the_fingerprint
---

# Signature

`fn fp(size: u64) -> FileFingerprint`

# Called by

- [matching_fingerprint_after_record_is_ok](../../../../../functions/src/llm/tools/file_read_cache/matching_fingerprint_after_record_is_ok.md)
- [mismatched_fingerprint_is_stale](../../../../../functions/src/llm/tools/file_read_cache/mismatched_fingerprint_is_stale.md)
- [distinct_paths_are_tracked_independently](../../../../../functions/src/llm/tools/file_read_cache/distinct_paths_are_tracked_independently.md)
- [re_recording_updates_the_fingerprint](../../../../../functions/src/llm/tools/file_read_cache/re_recording_updates_the_fingerprint.md)