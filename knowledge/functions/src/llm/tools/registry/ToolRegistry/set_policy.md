---
type: Rust Method
title: set_policy
resource: src/llm/tools/registry.rs#L50-L52
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/cli/cmd_chat
  - functions/src/cli/cmd_run
  - functions/src/llm/tools/registry/execute_evaluates_policy_against_the_canonical_name_not_the_alias
  - functions/src/llm/tools/registry/is_trusted_evaluates_policy_against_the_canonical_name_not_the_alias
---

# Signature

`pub fn set_policy(&mut self, policy: Arc<dyn crate::llm::tools::sandbox::PermissionPolicy>)`

# Called by

- [cmd_chat](../../../../../../functions/src/cli/cmd_chat.md)
- [cmd_run](../../../../../../functions/src/cli/cmd_run.md)
- [execute_evaluates_policy_against_the_canonical_name_not_the_alias](../../../../../../functions/src/llm/tools/registry/execute_evaluates_policy_against_the_canonical_name_not_the_alias.md)
- [is_trusted_evaluates_policy_against_the_canonical_name_not_the_alias](../../../../../../functions/src/llm/tools/registry/is_trusted_evaluates_policy_against_the_canonical_name_not_the_alias.md)