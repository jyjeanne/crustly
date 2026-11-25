---
type: Rust Method
title: new
resource: src/llm/tools/sandbox.rs#L50-L54
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/config/SecurityConfig/to_policy
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/registry/is_denied_follows_the_policy_through_aliases
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/registry/execute_evaluates_policy_against_the_canonical_name_not_the_alias
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/sandbox/and_policy_short_circuits_on_deny
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/sandbox/and_policy_denies_trusted_command_that_a_later_rule_rejects
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/sandbox/and_policy_preserves_trust_when_no_rule_denies
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn new(pattern: &str) -> Self`

# Called by

- [to_policy](../../../../../../functions/src/config/SecurityConfig/to_policy.md)
- [is_denied_follows_the_policy_through_aliases](../../../../../../functions/src/llm/tools/registry/is_denied_follows_the_policy_through_aliases.md)
- [execute_evaluates_policy_against_the_canonical_name_not_the_alias](../../../../../../functions/src/llm/tools/registry/execute_evaluates_policy_against_the_canonical_name_not_the_alias.md)
- [and_policy_short_circuits_on_deny](../../../../../../functions/src/llm/tools/sandbox/and_policy_short_circuits_on_deny.md)
- [and_policy_denies_trusted_command_that_a_later_rule_rejects](../../../../../../functions/src/llm/tools/sandbox/and_policy_denies_trusted_command_that_a_later_rule_rejects.md)
- [and_policy_preserves_trust_when_no_rule_denies](../../../../../../functions/src/llm/tools/sandbox/and_policy_preserves_trust_when_no_rule_denies.md)