---
type: Rust Module
title: file_read_cache
resource: src/llm/tools/file_read_cache.rs#L1-L141
generated:
  by: okf-rs/0.3.0
relationships:
  imports:
  - external/std-collections-hashmap
  - external/std-path-path-pathbuf
  - external/std-sync-mutex
  - external/std-time-systemtime
  - external/super
  member_of:
  - packages/crustly
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