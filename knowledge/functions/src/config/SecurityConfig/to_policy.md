---
type: Rust Method
title: to_policy
resource: src/config/mod.rs#L67-L93
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/tools/sandbox/DenyToolRule/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/sandbox/DenyPathPrefixRule/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/cli/cmd_chat
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/cmd_run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/allow_bash_trusts_only_the_listed_read_only_programs
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/empty_security_config_trusts_nothing
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn to_policy(&self) -> Box<dyn crate::llm::tools::sandbox::PermissionPolicy>`

# Calls

- [new](../../../../functions/src/llm/tools/sandbox/DenyToolRule/new.md)
- [new](../../../../functions/src/llm/tools/sandbox/DenyPathPrefixRule/new.md)

# Called by

- [cmd_chat](../../../../functions/src/cli/cmd_chat.md)
- [cmd_run](../../../../functions/src/cli/cmd_run.md)
- [allow_bash_trusts_only_the_listed_read_only_programs](../../../../functions/src/config/allow_bash_trusts_only_the_listed_read_only_programs.md)
- [empty_security_config_trusts_nothing](../../../../functions/src/config/empty_security_config_trusts_nothing.md)