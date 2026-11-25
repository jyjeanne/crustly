---
type: Rust Module
title: file_read_cache
resource: src/llm/tools/file_read_cache.rs#L1-L141
generated:
  by: okf-rs/0.8.0
relationships:
  imports:
  - target: external/std-collections-hashmap
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-path-pathbuf
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-sync-mutex
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-time-systemtime
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [FileFingerprint](../../../../classes/src/llm/tools/file_read_cache/FileFingerprint.md)
- [of](../../../../functions/src/llm/tools/file_read_cache/FileFingerprint/of.md)
- [ReadGate](../../../../classes/src/llm/tools/file_read_cache/ReadGate.md)
- [FileReadCache](../../../../classes/src/llm/tools/file_read_cache/FileReadCache.md)
- [new](../../../../functions/src/llm/tools/file_read_cache/FileReadCache/new.md)
- [record](../../../../functions/src/llm/tools/file_read_cache/FileReadCache/record.md)
- [check](../../../../functions/src/llm/tools/file_read_cache/FileReadCache/check.md)
- [fp](../../../../functions/src/llm/tools/file_read_cache/fp.md)
- [never_read_path_is_rejected](../../../../functions/src/llm/tools/file_read_cache/never_read_path_is_rejected.md)
- [matching_fingerprint_after_record_is_ok](../../../../functions/src/llm/tools/file_read_cache/matching_fingerprint_after_record_is_ok.md)
- [mismatched_fingerprint_is_stale](../../../../functions/src/llm/tools/file_read_cache/mismatched_fingerprint_is_stale.md)
- [distinct_paths_are_tracked_independently](../../../../functions/src/llm/tools/file_read_cache/distinct_paths_are_tracked_independently.md)
- [re_recording_updates_the_fingerprint](../../../../functions/src/llm/tools/file_read_cache/re_recording_updates_the_fingerprint.md)

# Imports

- `std::collections::HashMap`
- `std::path::{Path, PathBuf}`
- `std::sync::Mutex`
- `std::time::SystemTime`
- `super::*`

# Member of

- [crustly](../../../../packages/crustly.md)