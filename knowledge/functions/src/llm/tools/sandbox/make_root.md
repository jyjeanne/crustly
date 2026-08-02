---
type: Rust Function
title: make_root
resource: src/llm/tools/sandbox.rs#L487-L491
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/llm/tools/sandbox/absolute_path_outside_root_denied
  - functions/src/llm/tools/sandbox/valid_path_inside_root_allowed
  - functions/src/llm/tools/sandbox/path_traversal_denied
  - functions/src/llm/tools/sandbox/deny_path_prefix_allows_unrelated_path
  - functions/src/llm/tools/sandbox/symlink_outside_root_denied
---

# Signature

`fn make_root() -> (TempDir, PathBuf)`

# Called by

- [absolute_path_outside_root_denied](../../../../../functions/src/llm/tools/sandbox/absolute_path_outside_root_denied.md)
- [valid_path_inside_root_allowed](../../../../../functions/src/llm/tools/sandbox/valid_path_inside_root_allowed.md)
- [path_traversal_denied](../../../../../functions/src/llm/tools/sandbox/path_traversal_denied.md)
- [deny_path_prefix_allows_unrelated_path](../../../../../functions/src/llm/tools/sandbox/deny_path_prefix_allows_unrelated_path.md)
- [symlink_outside_root_denied](../../../../../functions/src/llm/tools/sandbox/symlink_outside_root_denied.md)