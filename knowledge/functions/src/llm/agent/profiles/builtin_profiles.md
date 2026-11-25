---
type: Rust Function
title: builtin_profiles
resource: src/llm/agent/profiles.rs#L34-L99
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/agent/profiles/resolve_profiles
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/profiles/the_built_in_profiles_exist_and_explorers_cannot_write
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/new
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn builtin_profiles() -> BTreeMap<String, AgentProfile>`

# Called by

- [resolve_profiles](../../../../../functions/src/llm/agent/profiles/resolve_profiles.md)
- [the_built_in_profiles_exist_and_explorers_cannot_write](../../../../../functions/src/llm/agent/profiles/the_built_in_profiles_exist_and_explorers_cannot_write.md)
- [new](../../../../../functions/src/llm/agent/service/AgentService/new.md)