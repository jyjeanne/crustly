---
type: Rust Function
title: resolve_profiles
resource: src/llm/agent/profiles.rs#L103-L129
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/profiles/builtin_profiles
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/configure_agent
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/profiles/configuration_overrides_fields_and_adds_profiles
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn resolve_profiles( configured: &HashMap<String, AgentProfileConfig>, ) -> BTreeMap<String, AgentProfile>`

# Calls

- [builtin_profiles](../../../../../functions/src/llm/agent/profiles/builtin_profiles.md)

# Called by

- [configure_agent](../../../../../functions/src/cli/configure_agent.md)
- [configuration_overrides_fields_and_adds_profiles](../../../../../functions/src/llm/agent/profiles/configuration_overrides_fields_and_adds_profiles.md)