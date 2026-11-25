---
type: Rust Module
title: project
resource: src/memory/project.rs#L1-L248
generated:
  by: okf-rs/0.8.0
relationships:
  imports:
  - target: external/super-model-projectid
    resolved_by: tree-sitter
    confidence: exact
  - target: external/sha2-digest-sha256
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-fs
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-path-pathbuf
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

- [ProjectIdentity](../../../classes/src/memory/project/ProjectIdentity.md)
- [id_for_root](../../../functions/src/memory/project/id_for_root.md)
- [identify](../../../functions/src/memory/project/identify.md)
- [find_root](../../../functions/src/memory/project/find_root.md)
- [normalize_root](../../../functions/src/memory/project/normalize_root.md)
- [origin_url](../../../functions/src/memory/project/origin_url.md)
- [sha256_hex](../../../functions/src/memory/project/sha256_hex.md)
- [write](../../../functions/src/memory/project/write.md)
- [a_plain_directory_is_its_own_project](../../../functions/src/memory/project/a_plain_directory_is_its_own_project.md)
- [a_subdirectory_resolves_to_its_repository_root](../../../functions/src/memory/project/a_subdirectory_resolves_to_its_repository_root.md)
- [a_linked_worktree_belongs_to_the_main_repository](../../../functions/src/memory/project/a_linked_worktree_belongs_to_the_main_repository.md)
- [a_worktree_is_found_through_its_commondir_file](../../../functions/src/memory/project/a_worktree_is_found_through_its_commondir_file.md)
- [a_submodule_under_a_worktrees_directory_is_its_own_project](../../../functions/src/memory/project/a_submodule_under_a_worktrees_directory_is_its_own_project.md)
- [the_origin_url_is_read_for_display](../../../functions/src/memory/project/the_origin_url_is_read_for_display.md)
- [two_clones_of_one_remote_are_two_projects](../../../functions/src/memory/project/two_clones_of_one_remote_are_two_projects.md)
- [normalize_root_strips_the_verbatim_prefix_and_lowers_the_drive](../../../functions/src/memory/project/normalize_root_strips_the_verbatim_prefix_and_lowers_the_drive.md)

# Imports

- `super::model::ProjectId`
- `sha2::{Digest, Sha256}`
- `std::fs`
- `std::path::{Path, PathBuf}`
- `super::*`

# Member of

- [crustly](../../../packages/crustly.md)