---
type: Rust Module
title: paths
resource: src/memory/paths.rs#L1-L202
generated:
  by: okf-rs/0.8.0
relationships:
  imports:
  - target: external/glob-matchoptions-pattern
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-component-path-pathbuf
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

- [validate_glob](../../../functions/src/memory/paths/validate_glob.md)
- [mentioned_paths](../../../functions/src/memory/paths/mentioned_paths.md)
- [project_relative](../../../functions/src/memory/paths/project_relative.md)
- [lexical](../../../functions/src/memory/paths/lexical.md)
- [as_text](../../../functions/src/memory/paths/as_text.md)
- [strip_root](../../../functions/src/memory/paths/strip_root.md)
- [matches](../../../functions/src/memory/paths/matches.md)
- [valid_relative_globs_are_normalised](../../../functions/src/memory/paths/valid_relative_globs_are_normalised.md)
- [globs_that_leave_the_project_are_refused](../../../functions/src/memory/paths/globs_that_leave_the_project_are_refused.md)
- [paths_are_found_in_a_message](../../../functions/src/memory/paths/paths_are_found_in_a_message.md)
- [globs_match_paths_and_stop_at_directories](../../../functions/src/memory/paths/globs_match_paths_and_stop_at_directories.md)

# Imports

- `glob::{MatchOptions, Pattern}`
- `std::path::{Component, Path, PathBuf}`
- `super::*`

# Member of

- [crustly](../../../packages/crustly.md)