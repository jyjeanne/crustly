---
type: Rust Method
title: evaluate
resource: src/llm/tools/sandbox.rs#L249-L286
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/tools/sandbox/find_active_shell_operator
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/events/EventHandler/next
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/sandbox/bash_allowlist_trusts_listed_prompts_for_unlisted
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/sandbox/bash_allowlist_allows_unlisted_operator_free_command_to_reach_approval
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/sandbox/bash_allowlist_never_trusts_shell_operator_chaining
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/sandbox/bash_allowlist_permits_quoted_operator_characters
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn evaluate(&self, tool_name: &str, inputs: &Value) -> PolicyDecision`

# Calls

- [find_active_shell_operator](../../../../../../../functions/src/llm/tools/sandbox/find_active_shell_operator.md)
- [next](../../../../../../../functions/src/tui/events/EventHandler/next.md)

# Called by

- [bash_allowlist_trusts_listed_prompts_for_unlisted](../../../../../../../functions/src/llm/tools/sandbox/bash_allowlist_trusts_listed_prompts_for_unlisted.md)
- [bash_allowlist_allows_unlisted_operator_free_command_to_reach_approval](../../../../../../../functions/src/llm/tools/sandbox/bash_allowlist_allows_unlisted_operator_free_command_to_reach_approval.md)
- [bash_allowlist_never_trusts_shell_operator_chaining](../../../../../../../functions/src/llm/tools/sandbox/bash_allowlist_never_trusts_shell_operator_chaining.md)
- [bash_allowlist_permits_quoted_operator_characters](../../../../../../../functions/src/llm/tools/sandbox/bash_allowlist_permits_quoted_operator_characters.md)