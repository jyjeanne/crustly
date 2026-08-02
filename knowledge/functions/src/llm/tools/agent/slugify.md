---
type: Rust Function
title: slugify
resource: src/llm/tools/agent.rs#L206-L220
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/is_empty
  called_by:
  - functions/src/llm/tools/agent/AgentTool/tool/execute
---

# Signature

`fn slugify(s: &str) -> String`

# Calls

- [is_empty](../../../../../functions/src/config/secrets/SecretString/is_empty.md)

# Called by

- [execute](../../../../../functions/src/llm/tools/agent/AgentTool/tool/execute.md)