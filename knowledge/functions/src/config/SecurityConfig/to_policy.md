---
type: Rust Method
title: to_policy
resource: src/config/mod.rs#L67-L93
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/is_empty
  called_by:
  - functions/src/cli/cmd_chat
  - functions/src/cli/cmd_run
  - functions/src/config/allow_bash_trusts_only_the_listed_read_only_programs
  - functions/src/config/empty_security_config_trusts_nothing
---

# Signature

`pub fn to_policy(&self) -> Box<dyn crate::llm::tools::sandbox::PermissionPolicy>`

# Calls

- [is_empty](../../../../functions/src/config/secrets/SecretString/is_empty.md)

# Called by

- [cmd_chat](../../../../functions/src/cli/cmd_chat.md)
- [cmd_run](../../../../functions/src/cli/cmd_run.md)
- [allow_bash_trusts_only_the_listed_read_only_programs](../../../../functions/src/config/allow_bash_trusts_only_the_listed_read_only_programs.md)
- [empty_security_config_trusts_nothing](../../../../functions/src/config/empty_security_config_trusts_nothing.md)