---
type: Rust Module
title: sandbox
resource: src/llm/tools/sandbox.rs#L1-L885
generated:
  by: okf-rs/0.2.0
relationships:
  imports:
  - external/serde-json-value
  - external/std-path-path-pathbuf
  - external/std-path-component-prefix
  - external/super
  - external/tempfile-tempdir
  - external/std-sync-atomic-atomicbool-ordering
  - external/std-sync-arc
  member_of:
  - packages/crustly
---

# Contains

- [PolicyDecision](../../../../classes/src/llm/tools/sandbox/PolicyDecision.md)
- [is_permitted](../../../../functions/src/llm/tools/sandbox/PolicyDecision/is_permitted.md)
- [PermissionPolicy](../../../../interfaces/src/llm/tools/sandbox/PermissionPolicy.md)
- [DenyToolRule](../../../../classes/src/llm/tools/sandbox/DenyToolRule.md)
- [new](../../../../functions/src/llm/tools/sandbox/DenyToolRule/new.md)
- [evaluate](../../../../functions/src/llm/tools/sandbox/DenyToolRule/permissionpolicy/evaluate.md)
- [AllowToolRule](../../../../classes/src/llm/tools/sandbox/AllowToolRule.md)
- [new](../../../../functions/src/llm/tools/sandbox/AllowToolRule/new.md)
- [evaluate](../../../../functions/src/llm/tools/sandbox/AllowToolRule/permissionpolicy/evaluate.md)
- [DenyPathPrefixRule](../../../../classes/src/llm/tools/sandbox/DenyPathPrefixRule.md)
- [new](../../../../functions/src/llm/tools/sandbox/DenyPathPrefixRule/new.md)
- [evaluate](../../../../functions/src/llm/tools/sandbox/DenyPathPrefixRule/permissionpolicy/evaluate.md)
- [PathBoundaryRule](../../../../classes/src/llm/tools/sandbox/PathBoundaryRule.md)
- [check](../../../../functions/src/llm/tools/sandbox/PathBoundaryRule/check.md)
- [strip_verbatim_prefix](../../../../functions/src/llm/tools/sandbox/strip_verbatim_prefix.md)
- [evaluate](../../../../functions/src/llm/tools/sandbox/PathBoundaryRule/permissionpolicy/evaluate.md)
- [BashCommandAllowlist](../../../../classes/src/llm/tools/sandbox/BashCommandAllowlist.md)
- [evaluate](../../../../functions/src/llm/tools/sandbox/BashCommandAllowlist/permissionpolicy/evaluate.md)
- [find_active_shell_operator](../../../../functions/src/llm/tools/sandbox/find_active_shell_operator.md)
- [AndPolicy](../../../../classes/src/llm/tools/sandbox/AndPolicy.md)
- [evaluate](../../../../functions/src/llm/tools/sandbox/AndPolicy/permissionpolicy/evaluate.md)
- [OrPolicy](../../../../classes/src/llm/tools/sandbox/OrPolicy.md)
- [evaluate](../../../../functions/src/llm/tools/sandbox/OrPolicy/permissionpolicy/evaluate.md)
- [NotPolicy](../../../../classes/src/llm/tools/sandbox/NotPolicy.md)
- [evaluate](../../../../functions/src/llm/tools/sandbox/NotPolicy/permissionpolicy/evaluate.md)
- [AllowAll](../../../../classes/src/llm/tools/sandbox/AllowAll.md)
- [evaluate](../../../../functions/src/llm/tools/sandbox/AllowAll/permissionpolicy/evaluate.md)
- [check_path](../../../../functions/src/llm/tools/sandbox/check_path.md)
- [normalize_path](../../../../functions/src/llm/tools/sandbox/normalize_path.md)
- [resolve_existing_prefix](../../../../functions/src/llm/tools/sandbox/resolve_existing_prefix.md)
- [make_root](../../../../functions/src/llm/tools/sandbox/make_root.md)
- [absolute_path_outside_root_denied](../../../../functions/src/llm/tools/sandbox/absolute_path_outside_root_denied.md)
- [absolute_path_to_nonexistent_file_inside_root_allowed](../../../../functions/src/llm/tools/sandbox/absolute_path_to_nonexistent_file_inside_root_allowed.md)
- [absolute_path_to_nonexistent_file_in_subdir_allowed](../../../../functions/src/llm/tools/sandbox/absolute_path_to_nonexistent_file_in_subdir_allowed.md)
- [absolute_path_to_nonexistent_file_outside_root_still_denied](../../../../functions/src/llm/tools/sandbox/absolute_path_to_nonexistent_file_outside_root_still_denied.md)
- [absolute_path_to_nonexistent_file_through_a_symlinked_root_allowed](../../../../functions/src/llm/tools/sandbox/absolute_path_to_nonexistent_file_through_a_symlinked_root_allowed.md)
- [valid_path_inside_root_allowed](../../../../functions/src/llm/tools/sandbox/valid_path_inside_root_allowed.md)
- [and_policy_short_circuits_on_deny](../../../../functions/src/llm/tools/sandbox/and_policy_short_circuits_on_deny.md)
- [PanicIfCalled](../../../../classes/src/llm/tools/sandbox/PanicIfCalled.md)
- [evaluate](../../../../functions/src/llm/tools/sandbox/PanicIfCalled/permissionpolicy/evaluate.md)
- [or_policy_short_circuits_on_allow](../../../../functions/src/llm/tools/sandbox/or_policy_short_circuits_on_allow.md)
- [PanicIfCalled](../../../../classes/src/llm/tools/sandbox/PanicIfCalled-2.md)
- [evaluate](../../../../functions/src/llm/tools/sandbox/PanicIfCalled/permissionpolicy/evaluate-2.md)
- [bash_allowlist_trusts_listed_prompts_for_unlisted](../../../../functions/src/llm/tools/sandbox/bash_allowlist_trusts_listed_prompts_for_unlisted.md)
- [bash_allowlist_allows_unlisted_operator_free_command_to_reach_approval](../../../../functions/src/llm/tools/sandbox/bash_allowlist_allows_unlisted_operator_free_command_to_reach_approval.md)
- [bash_allowlist_never_trusts_shell_operator_chaining](../../../../functions/src/llm/tools/sandbox/bash_allowlist_never_trusts_shell_operator_chaining.md)
- [bash_allowlist_permits_quoted_operator_characters](../../../../functions/src/llm/tools/sandbox/bash_allowlist_permits_quoted_operator_characters.md)
- [allow_all_never_confers_trust](../../../../functions/src/llm/tools/sandbox/allow_all_never_confers_trust.md)
- [and_policy_denies_trusted_command_that_a_later_rule_rejects](../../../../functions/src/llm/tools/sandbox/and_policy_denies_trusted_command_that_a_later_rule_rejects.md)
- [and_policy_preserves_trust_when_no_rule_denies](../../../../functions/src/llm/tools/sandbox/and_policy_preserves_trust_when_no_rule_denies.md)
- [and_policy_does_not_trust_unlisted_program](../../../../functions/src/llm/tools/sandbox/and_policy_does_not_trust_unlisted_program.md)
- [not_policy_inverts_trusted_to_deny](../../../../functions/src/llm/tools/sandbox/not_policy_inverts_trusted_to_deny.md)
- [not_policy_inverts_allow](../../../../functions/src/llm/tools/sandbox/not_policy_inverts_allow.md)
- [path_traversal_denied](../../../../functions/src/llm/tools/sandbox/path_traversal_denied.md)
- [deny_path_prefix_blocks_matching_path](../../../../functions/src/llm/tools/sandbox/deny_path_prefix_blocks_matching_path.md)
- [deny_path_prefix_allows_unrelated_path](../../../../functions/src/llm/tools/sandbox/deny_path_prefix_allows_unrelated_path.md)
- [symlink_outside_root_denied](../../../../functions/src/llm/tools/sandbox/symlink_outside_root_denied.md)

# Imports

- `serde_json::Value`
- `std::path::{Path, PathBuf}`
- `std::path::{Component, Prefix}`
- `super::*`
- `tempfile::TempDir`
- `std::sync::atomic::{AtomicBool, Ordering}`
- `std::sync::Arc`

# Member of

- [crustly](../../../../packages/crustly.md)